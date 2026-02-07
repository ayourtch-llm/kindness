use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::Instant;
use tracing::{error, info, warn};

// ── CLI ────────────────────────────────────────────────────────────────────

#[derive(Parser)]
#[command(name = "runner", about = "Experiment runner for LLM prompting style study")]
struct Cli {
    /// OpenAI-compatible chat completions endpoint
    #[arg(long, default_value = "https://api.openai.com/v1/chat/completions")]
    api_url: String,

    /// Model name to request
    #[arg(long, default_value = "gpt-4o")]
    model: String,

    /// Number of repetitions per task×style combination
    #[arg(long, default_value_t = 3)]
    repetitions: u32,

    /// Print rendered prompts without calling the API
    #[arg(long)]
    dry_run: bool,

    /// Glob pattern to filter task IDs (e.g. "3_*")
    #[arg(long)]
    tasks: Option<String>,

    /// Comma-separated list of styles to run
    #[arg(long)]
    styles: Option<String>,

    /// Maximum tokens for the completion
    #[arg(long, default_value_t = 4096)]
    max_tokens: u32,

    /// Sampling temperature (0.0 for reproducibility)
    #[arg(long, default_value_t = 0.0)]
    temperature: f64,
}

// ── Task TOML schema ───────────────────────────────────────────────────────

#[derive(Deserialize)]
struct TaskFile {
    task: Task,
}

#[derive(Deserialize)]
struct Task {
    id: String,
    #[allow(dead_code)]
    tier: u32,
    #[allow(dead_code)]
    title: String,
    #[allow(dead_code)]
    language: String,
    description: Description,
    constraints: Constraints,
    #[allow(dead_code)]
    tests: Tests,
}

#[derive(Deserialize)]
struct Description {
    text: String,
}

#[derive(Deserialize)]
struct Constraints {
    #[allow(dead_code)]
    time_limit_seconds: u64,
    #[serde(default)]
    must_use: Vec<String>,
    #[serde(default)]
    forbidden: Vec<String>,
}

#[derive(Deserialize)]
struct Tests {
    #[allow(dead_code)]
    cases: Vec<TestCase>,
}

#[derive(Deserialize)]
struct TestCase {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    code: String,
}

// ── OpenAI-compatible API types ────────────────────────────────────────────

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    max_tokens: u32,
    temperature: f64,
}

#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
    usage: Option<Usage>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageContent,
}

#[derive(Deserialize)]
struct MessageContent {
    content: String,
}

#[derive(Deserialize)]
struct Usage {
    prompt_tokens: u64,
    completion_tokens: u64,
    total_tokens: u64,
}

// ── Result metadata ────────────────────────────────────────────────────────

#[derive(Serialize)]
struct RunMetadata {
    task_id: String,
    style: String,
    run: u32,
    model: String,
    api_url: String,
    timestamp: String,
    latency_ms: u128,
    input_tokens: u64,
    output_tokens: u64,
    total_tokens: u64,
}

// ── Constants ──────────────────────────────────────────────────────────────

const ALL_STYLES: &[&str] = &["personified", "polite_directive", "bare_directive"];
const MAX_RETRIES: u32 = 5;
const BASE_RETRY_MS: u64 = 1000;

// ── Main ───────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    let project_root = find_project_root()?;
    info!("Project root: {}", project_root.display());

    // Resolve API key (not needed for dry-run)
    let api_key = if cli.dry_run {
        String::new()
    } else {
        std::env::var("LLM_API_KEY").map_err(|_| {
            "LLM_API_KEY environment variable not set. Set it or use --dry-run."
        })?
    };

    // Load system prompt
    let system_prompt_path = project_root.join("prompts/templates/system.md");
    let system_prompt = std::fs::read_to_string(&system_prompt_path).map_err(|e| {
        format!(
            "Failed to read system prompt at {}: {}",
            system_prompt_path.display(),
            e
        )
    })?;
    let system_prompt = system_prompt.trim().to_string();

    // Determine styles
    let styles: Vec<String> = match &cli.styles {
        Some(s) => s.split(',').map(|v| v.trim().to_string()).collect(),
        None => ALL_STYLES.iter().map(|s| s.to_string()).collect(),
    };

    // Load style templates
    let mut templates: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();
    for style in &styles {
        let tpl_path = project_root.join(format!("prompts/templates/{}.md", style));
        let tpl = std::fs::read_to_string(&tpl_path).map_err(|e| {
            format!(
                "Failed to read template {}: {}",
                tpl_path.display(),
                e
            )
        })?;
        templates.insert(style.clone(), tpl);
    }

    // Discover and load tasks
    let tasks = load_tasks(&project_root, cli.tasks.as_deref())?;
    info!("Loaded {} task(s), {} style(s)", tasks.len(), styles.len());

    if tasks.is_empty() {
        warn!("No tasks matched. Exiting.");
        return Ok(());
    }

    let client = reqwest::Client::new();

    for task_file in &tasks {
        let task = &task_file.task;
        let constraints_block = build_constraints_block(&task.constraints);

        for style in &styles {
            let template = &templates[style];
            let rendered = render_template(template, &task.description.text, &constraints_block);

            for run in 1..=cli.repetitions {
                info!(
                    "Task={} Style={} Run={}/{}",
                    task.id, style, run, cli.repetitions
                );

                if cli.dry_run {
                    println!("─── DRY RUN: {} / {} / run {} ───", task.id, style, run);
                    println!("[system]\n{}\n", system_prompt);
                    println!("[user]\n{}\n", rendered);
                    continue;
                }

                let result = call_api_with_retry(
                    &client,
                    &cli.api_url,
                    &api_key,
                    &cli.model,
                    &system_prompt,
                    &rendered,
                    cli.max_tokens,
                    cli.temperature,
                )
                .await;

                match result {
                    Ok((response, latency_ms)) => {
                        let code = &response.choices[0].message.content;
                        let usage = response.usage.as_ref();

                        let meta = RunMetadata {
                            task_id: task.id.clone(),
                            style: style.clone(),
                            run,
                            model: cli.model.clone(),
                            api_url: cli.api_url.clone(),
                            timestamp: chrono_now(),
                            latency_ms,
                            input_tokens: usage.map_or(0, |u| u.prompt_tokens),
                            output_tokens: usage.map_or(0, |u| u.completion_tokens),
                            total_tokens: usage.map_or(0, |u| u.total_tokens),
                        };

                        if let Err(e) =
                            save_results(&project_root, &task.id, style, run, code, &meta)
                        {
                            error!("Failed to save results: {}", e);
                        }
                    }
                    Err(e) => {
                        error!(
                            "API call failed for {} / {} / run {}: {}",
                            task.id, style, run, e
                        );
                    }
                }
            }
        }
    }

    info!("Done.");
    Ok(())
}

// ── Helpers ────────────────────────────────────────────────────────────────

fn find_project_root() -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Walk up from the current directory looking for the tasks/ directory
    let mut dir = std::env::current_dir()?;
    loop {
        if dir.join("tasks").is_dir() && dir.join("prompts").is_dir() {
            return Ok(dir);
        }
        if !dir.pop() {
            break;
        }
    }
    // Fallback: try parent of runner/
    let fallback = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let fallback = fallback.canonicalize()?;
    if fallback.join("tasks").is_dir() {
        return Ok(fallback);
    }
    Err("Could not find project root (directory containing tasks/ and prompts/)".into())
}

fn load_tasks(
    root: &Path,
    filter: Option<&str>,
) -> Result<Vec<TaskFile>, Box<dyn std::error::Error>> {
    let defs_dir = root.join("tasks/definitions");
    let pattern = defs_dir.join("*.toml").to_string_lossy().to_string();
    let mut tasks = Vec::new();

    for entry in glob::glob(&pattern)? {
        let path = entry?;
        let content = std::fs::read_to_string(&path)?;
        let task_file: TaskFile = toml::from_str(&content).map_err(|e| {
            format!("Failed to parse {}: {}", path.display(), e)
        })?;

        // Apply task ID glob filter
        if let Some(filt) = filter {
            let pat = glob::Pattern::new(filt)
                .map_err(|e| format!("Invalid --tasks pattern '{}': {}", filt, e))?;
            if !pat.matches(&task_file.task.id) {
                continue;
            }
        }

        tasks.push(task_file);
    }

    tasks.sort_by(|a, b| a.task.id.cmp(&b.task.id));
    Ok(tasks)
}

fn build_constraints_block(constraints: &Constraints) -> String {
    let mut parts = Vec::new();
    if !constraints.must_use.is_empty() {
        parts.push(format!("Required: use {}.", constraints.must_use.join(", ")));
    }
    if !constraints.forbidden.is_empty() {
        parts.push(format!(
            "Do not use: {}.",
            constraints.forbidden.join(", ")
        ));
    }
    parts.join("\n")
}

fn render_template(template: &str, task_description: &str, constraints_block: &str) -> String {
    template
        .replace("{{task_description}}", task_description)
        .replace("{{constraints_block}}", constraints_block)
}

async fn call_api_with_retry(
    client: &reqwest::Client,
    api_url: &str,
    api_key: &str,
    model: &str,
    system_prompt: &str,
    user_prompt: &str,
    max_tokens: u32,
    temperature: f64,
) -> Result<(ChatResponse, u128), Box<dyn std::error::Error>> {
    let request_body = ChatRequest {
        model: model.to_string(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: user_prompt.to_string(),
            },
        ],
        max_tokens,
        temperature,
    };

    let body_json = serde_json::to_string(&request_body)?;

    for attempt in 0..MAX_RETRIES {
        let start = Instant::now();
        let resp = client
            .post(api_url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .body(body_json.clone())
            .send()
            .await?;

        let status = resp.status();
        let latency_ms = start.elapsed().as_millis();

        if status.is_success() {
            let chat_resp: ChatResponse = resp.json().await?;
            if chat_resp.choices.is_empty() {
                return Err("API returned empty choices array".into());
            }
            return Ok((chat_resp, latency_ms));
        }

        // Retry on transient errors
        if matches!(status.as_u16(), 429 | 500 | 502 | 503) {
            let delay = BASE_RETRY_MS * 2u64.pow(attempt);
            warn!(
                "Request failed with {} (attempt {}/{}), retrying in {}ms",
                status,
                attempt + 1,
                MAX_RETRIES,
                delay
            );
            tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
            continue;
        }

        // Non-retryable error
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("API error {}: {}", status, body).into());
    }

    Err(format!("Exhausted {} retries", MAX_RETRIES).into())
}

fn save_results(
    root: &Path,
    task_id: &str,
    style: &str,
    run: u32,
    code: &str,
    meta: &RunMetadata,
) -> Result<(), Box<dyn std::error::Error>> {
    let dir = root.join(format!("results/{}/{}", task_id, style));
    std::fs::create_dir_all(&dir)?;

    let code_path = dir.join(format!("run_{}.rs", run));
    std::fs::write(&code_path, code)?;
    info!("Saved code to {}", code_path.display());

    let meta_path = dir.join(format!("run_{}.meta.json", run));
    let meta_json = serde_json::to_string_pretty(meta)?;
    std::fs::write(&meta_path, meta_json)?;
    info!("Saved metadata to {}", meta_path.display());

    Ok(())
}

fn chrono_now() -> String {
    // ISO 8601 UTC timestamp without external chrono dependency
    use std::time::SystemTime;
    let duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();

    // Convert epoch seconds to date-time components
    let days = secs / 86400;
    let time_of_day = secs % 86400;
    let hours = time_of_day / 3600;
    let minutes = (time_of_day % 3600) / 60;
    let seconds = time_of_day % 60;

    // Days since 1970-01-01 to year/month/day
    let (year, month, day) = epoch_days_to_date(days);

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month, day, hours, minutes, seconds
    )
}

fn epoch_days_to_date(mut days: u64) -> (u64, u64, u64) {
    // Algorithm from Howard Hinnant's date library (public domain)
    days += 719_468;
    let era = days / 146_097;
    let doe = days % 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}
