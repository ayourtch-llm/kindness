use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
use tempfile::TempDir;
use walkdir::WalkDir;

// ── TOML task schema ────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct TaskFile {
    task: Task,
}

#[derive(Debug, Deserialize)]
struct Task {
    #[allow(dead_code)]
    id: String,
    #[allow(dead_code)]
    tier: u32,
    #[allow(dead_code)]
    title: String,
    #[allow(dead_code)]
    language: String,
    #[allow(dead_code)]
    description: TaskDescription,
    constraints: Constraints,
    tests: Tests,
}

#[derive(Debug, Deserialize)]
struct TaskDescription {
    #[allow(dead_code)]
    text: String,
}

#[derive(Debug, Deserialize)]
struct Constraints {
    time_limit_seconds: u64,
    #[serde(default)]
    must_use: Vec<String>,
    #[serde(default)]
    forbidden: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Tests {
    cases: Vec<TestCase>,
}

#[derive(Debug, Deserialize)]
struct TestCase {
    name: String,
    code: String,
}

// ── Meta JSON schema ────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct Meta {
    #[allow(dead_code)]
    task_id: String,
    #[allow(dead_code)]
    style: String,
    #[allow(dead_code)]
    run: u32,
    #[serde(default)]
    latency_ms: u64,
    #[serde(default)]
    input_tokens: u64,
    #[serde(default)]
    output_tokens: u64,
}

// ── Output schema ───────────────────────────────────────────────────

#[derive(Debug, Serialize)]
struct EvalResult {
    task_id: String,
    style: String,
    run: u32,
    compiled: bool,
    tests_total: usize,
    tests_passed: usize,
    tests_failed: Vec<String>,
    loc: usize,
    unsafe_count: usize,
    unwrap_count: usize,
    expect_count: usize,
    must_use_satisfied: bool,
    forbidden_violated: bool,
    latency_ms: u64,
    input_tokens: u64,
    output_tokens: u64,
}

// ── Helpers ─────────────────────────────────────────────────────────

/// Strip markdown code fences (```rust / ```) that LLMs sometimes emit.
fn strip_markdown_fences(raw: &str) -> String {
    let mut lines: Vec<&str> = Vec::new();
    let mut inside_fence = false;

    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            inside_fence = !inside_fence;
            continue;
        }
        lines.push(line);
    }
    // If there was an odd number of fences we still keep the collected lines.
    let _ = inside_fence;
    lines.join("\n")
}

/// Count logical lines of code (non-blank, non-comment-only).
fn count_loc(code: &str) -> usize {
    code.lines()
        .filter(|l| {
            let t = l.trim();
            !t.is_empty() && !t.starts_with("//")
        })
        .count()
}

/// Count occurrences of a literal pattern in code.
fn count_pattern(code: &str, pattern: &str) -> usize {
    code.matches(pattern).count()
}

/// Count `unsafe {` or `unsafe{` blocks.
fn count_unsafe(code: &str) -> usize {
    let re = Regex::new(r"unsafe\s*\{").expect("bad regex");
    re.find_iter(code).count()
}

/// Build a temporary crate, run `cargo test --release`, return (compiled, test_results).
fn run_tests(
    solution_code: &str,
    test_cases: &[TestCase],
    time_limit: Duration,
) -> (bool, HashMap<String, bool>) {
    let tmp = match TempDir::new() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("  ✗ could not create temp dir: {e}");
            let map: HashMap<String, bool> =
                test_cases.iter().map(|tc| (tc.name.clone(), false)).collect();
            return (false, map);
        }
    };

    let crate_dir = tmp.path();

    // Cargo.toml
    let cargo_toml = r#"[package]
name = "solution"
version = "0.1.0"
edition = "2021"
"#;
    let src_dir = crate_dir.join("src");
    let tests_dir = crate_dir.join("tests");
    if fs::create_dir_all(&src_dir).is_err() || fs::create_dir_all(&tests_dir).is_err() {
        let map: HashMap<String, bool> =
            test_cases.iter().map(|tc| (tc.name.clone(), false)).collect();
        return (false, map);
    }

    let write = |p: PathBuf, content: &str| -> bool {
        match fs::File::create(&p) {
            Ok(mut f) => f.write_all(content.as_bytes()).is_ok(),
            Err(_) => false,
        }
    };

    if !write(crate_dir.join("Cargo.toml"), cargo_toml) {
        let map: HashMap<String, bool> =
            test_cases.iter().map(|tc| (tc.name.clone(), false)).collect();
        return (false, map);
    }
    if !write(src_dir.join("lib.rs"), solution_code) {
        let map: HashMap<String, bool> =
            test_cases.iter().map(|tc| (tc.name.clone(), false)).collect();
        return (false, map);
    }

    // Build integration test file
    let mut integration = String::from("use solution::*;\n\n");
    for tc in test_cases {
        integration.push_str(&format!("#[test]\nfn {}() {{\n{}\n}}\n\n", tc.name, tc.code));
    }
    if !write(tests_dir.join("integration.rs"), &integration) {
        let map: HashMap<String, bool> =
            test_cases.iter().map(|tc| (tc.name.clone(), false)).collect();
        return (false, map);
    }

    // Run cargo test --release with timeout
    let child = Command::new("cargo")
        .args(["test", "--release"])
        .current_dir(crate_dir)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn();

    let mut child = match child {
        Ok(c) => c,
        Err(e) => {
            eprintln!("  ✗ failed to spawn cargo: {e}");
            let map: HashMap<String, bool> =
                test_cases.iter().map(|tc| (tc.name.clone(), false)).collect();
            return (false, map);
        }
    };

    // Wait with timeout
    let status = wait_with_timeout(&mut child, time_limit);

    let (compiled, stdout_str, stderr_str) = match status {
        Some(output) => {
            let stdout_s = String::from_utf8_lossy(&output.0).to_string();
            let stderr_s = String::from_utf8_lossy(&output.1).to_string();
            let compiled = !stderr_s.contains("could not compile")
                && !stderr_s.contains("error[E");
            (compiled, stdout_s, stderr_s)
        }
        None => {
            // Timeout – kill
            let _ = child.kill();
            let _ = child.wait();
            eprintln!("  ✗ timeout");
            let map: HashMap<String, bool> =
                test_cases.iter().map(|tc| (tc.name.clone(), false)).collect();
            return (false, map);
        }
    };

    if !compiled {
        eprintln!("  ✗ compilation failed");
        let combined = format!("{stdout_str}\n{stderr_str}");
        for line in combined.lines().filter(|l| l.contains("error")) {
            eprintln!("    {line}");
        }
        let map: HashMap<String, bool> =
            test_cases.iter().map(|tc| (tc.name.clone(), false)).collect();
        return (false, map);
    }

    // Parse test results from stdout
    let mut results: HashMap<String, bool> = HashMap::new();
    let combined = format!("{stdout_str}\n{stderr_str}");
    let re = Regex::new(r"test (\S+)\s+\.\.\.\s+(ok|FAILED)").expect("bad regex");
    for cap in re.captures_iter(&combined) {
        let name = cap[1].to_string();
        let passed = &cap[2] == "ok";
        results.insert(name, passed);
    }

    // Any test not found in output is treated as failed
    for tc in test_cases {
        results.entry(tc.name.clone()).or_insert(false);
    }

    (true, results)
}

/// Spawn-and-wait with a timeout, returning (stdout, stderr) on success.
fn wait_with_timeout(
    child: &mut std::process::Child,
    timeout: Duration,
) -> Option<(Vec<u8>, Vec<u8>)> {
    // Read pipes in separate threads to avoid deadlock when buffers fill.
    let stdout_handle = child.stdout.take().map(|pipe| {
        std::thread::spawn(move || {
            let mut buf = Vec::new();
            let _ = std::io::Read::read_to_end(&mut { pipe }, &mut buf);
            buf
        })
    });
    let stderr_handle = child.stderr.take().map(|pipe| {
        std::thread::spawn(move || {
            let mut buf = Vec::new();
            let _ = std::io::Read::read_to_end(&mut { pipe }, &mut buf);
            buf
        })
    });

    let start = std::time::Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(_status)) => break,
            Ok(None) => {
                if start.elapsed() >= timeout {
                    return None;
                }
                std::thread::sleep(Duration::from_millis(100));
            }
            Err(_) => return None,
        }
    }

    let stdout = stdout_handle.and_then(|h| h.join().ok()).unwrap_or_default();
    let stderr = stderr_handle.and_then(|h| h.join().ok()).unwrap_or_default();
    Some((stdout, stderr))
}

// ── Main ────────────────────────────────────────────────────────────

fn main() {
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let results_dir = project_root.join("results");
    let tasks_dir = project_root.join("tasks").join("definitions");
    let output_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("results.json");

    if !results_dir.is_dir() {
        eprintln!("results/ directory not found at {}", results_dir.display());
        std::process::exit(1);
    }

    // Collect .rs files
    let rs_files: Vec<PathBuf> = WalkDir::new(&results_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension().map_or(false, |ext| ext == "rs")
        })
        .map(|e| e.into_path())
        .collect();

    if rs_files.is_empty() {
        eprintln!("No .rs result files found; writing empty results.");
        let empty: Vec<EvalResult> = Vec::new();
        fs::write(&output_path, serde_json::to_string_pretty(&empty).unwrap())
            .expect("failed to write results.json");
        return;
    }

    // Cache loaded tasks
    let mut task_cache: HashMap<String, TaskFile> = HashMap::new();
    let mut eval_results: Vec<EvalResult> = Vec::new();

    for rs_path in &rs_files {
        // Parse path: results/{task_id}/{style}/{run_n}.rs
        let rel = rs_path
            .strip_prefix(&results_dir)
            .expect("should be under results/");
        let components: Vec<&str> = rel
            .components()
            .filter_map(|c| {
                if let std::path::Component::Normal(s) = c {
                    s.to_str()
                } else {
                    None
                }
            })
            .collect();

        if components.len() != 3 {
            eprintln!(
                "Skipping {}: expected results/{{task_id}}/{{style}}/{{run}}.rs",
                rs_path.display()
            );
            continue;
        }

        let task_id = components[0].to_string();
        let style = components[1].to_string();
        let run_file = components[2]; // e.g. "1.rs"
        let run: u32 = run_file
            .trim_end_matches(".rs")
            .parse()
            .unwrap_or_else(|_| {
                eprintln!("Cannot parse run number from {run_file}, defaulting to 0");
                0
            });

        println!("▶ {task_id} / {style} / run {run}");

        // Load task definition
        if !task_cache.contains_key(&task_id) {
            let toml_path = tasks_dir.join(format!("{task_id}.toml"));
            if !toml_path.is_file() {
                eprintln!("  ✗ task file not found: {}", toml_path.display());
                continue;
            }
            let toml_str = fs::read_to_string(&toml_path).expect("read task toml");
            let parsed: TaskFile = toml::from_str(&toml_str).unwrap_or_else(|e| {
                panic!("Failed to parse {}: {e}", toml_path.display());
            });
            task_cache.insert(task_id.clone(), parsed);
        }
        let task_def = &task_cache[&task_id];

        // Read solution code
        let raw_code = match fs::read_to_string(rs_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("  ✗ cannot read {}: {e}", rs_path.display());
                continue;
            }
        };
        let code = strip_markdown_fences(&raw_code);

        // Read .meta.json
        let meta_path = rs_path.with_extension("meta.json");
        let meta: Option<Meta> = if meta_path.is_file() {
            match fs::read_to_string(&meta_path) {
                Ok(s) => serde_json::from_str(&s).ok(),
                Err(_) => None,
            }
        } else {
            None
        };

        // Static metrics
        let loc = count_loc(&code);
        let unsafe_count = count_unsafe(&code);
        let unwrap_count = count_pattern(&code, ".unwrap()");
        let expect_count = count_pattern(&code, ".expect(");

        let must_use_satisfied = task_def
            .task
            .constraints
            .must_use
            .iter()
            .all(|item| code.contains(item.as_str()));

        let forbidden_violated = task_def
            .task
            .constraints
            .forbidden
            .iter()
            .any(|item| code.contains(item.as_str()));

        // Run tests
        let timeout = Duration::from_secs(task_def.task.constraints.time_limit_seconds);
        let (compiled, test_results) =
            run_tests(&code, &task_def.task.tests.cases, timeout);

        let tests_total = task_def.task.tests.cases.len();
        let tests_failed: Vec<String> = task_def
            .task
            .tests
            .cases
            .iter()
            .filter(|tc| !test_results.get(&tc.name).copied().unwrap_or(false))
            .map(|tc| tc.name.clone())
            .collect();
        let tests_passed = tests_total - tests_failed.len();

        let status = if compiled {
            format!("{tests_passed}/{tests_total} tests passed")
        } else {
            "did not compile".to_string()
        };
        println!("  → {status}");

        eval_results.push(EvalResult {
            task_id,
            style,
            run,
            compiled,
            tests_total,
            tests_passed,
            tests_failed,
            loc,
            unsafe_count,
            unwrap_count,
            expect_count,
            must_use_satisfied,
            forbidden_violated,
            latency_ms: meta.as_ref().map_or(0, |m| m.latency_ms),
            input_tokens: meta.as_ref().map_or(0, |m| m.input_tokens),
            output_tokens: meta.as_ref().map_or(0, |m| m.output_tokens),
        });
    }

    // Write output
    let json = serde_json::to_string_pretty(&eval_results).expect("serialize results");
    fs::write(&output_path, &json).expect("write results.json");
    println!("\n✔ Wrote {} results to {}", eval_results.len(), output_path.display());
}
