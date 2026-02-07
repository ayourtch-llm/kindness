# GENERATE.md — LLM Prompting Style Study

You are bootstrapping a research project that investigates whether the **phrasing style** of prompts to LLM coding agents affects output quality. Specifically, three styles:

1. **Personified** — addresses the LLM as a peer (“Hey, what do you think about using a hash table here?”)
1. **Polite-directive** — gives instructions with “please” (“Please implement a hash table for this lookup.”)
1. **Bare-directive** — gives instructions without pleasantries (“Implement a hash table for this lookup.”)

Read this entire file before doing anything. Then execute all sections in order.

-----

## 1. Repository Structure

Create the following structure at the repo root:

```
tasks/
  definitions/       # Generated task TOML files
  solutions/         # Reference solutions (one dir per task)
prompts/
  templates/         # The three prompt style templates
runner/
  src/               # Rust binary: API caller + response collector
  Cargo.toml
eval/
  src/               # Rust binary: test runner + metrics
  Cargo.toml
analysis/
  analyze.py         # Statistical analysis script (run via uv)
  pyproject.toml     # uv project with dependencies
results/             # .gitkeep — raw API responses go here at experiment time
README.md
```

-----

## 2. Task Generation

Generate **50 coding tasks** spanning 5 difficulty tiers (10 tasks each):

|Tier|Label  |Description                                      |Example scope                         |
|----|-------|-------------------------------------------------|--------------------------------------|
|1   |trivial|Single function, no edge cases                   |Reverse a string, FizzBuzz            |
|2   |easy   |Single function with edge cases or moderate logic|Valid parentheses, binary search      |
|3   |medium |Multiple functions or data structures            |LRU cache, trie implementation        |
|4   |hard   |Algorithmic complexity, system design component  |Interval scheduling, concurrent queue |
|5   |expert |Multi-file, architectural decisions required     |Mini HTTP router, expression evaluator|

Each task is a TOML file in `tasks/definitions/` named `{tier}_{nn}_{slug}.toml`:

```toml
[task]
id = "3_01_lru_cache"
tier = 3
title = "LRU Cache"
language = "rust"

[task.description]
text = """
Implement an LRU cache with O(1) get and put operations.
The cache should have a configurable capacity.
"""

[task.constraints]
time_limit_seconds = 5
must_use = []          # e.g. ["HashMap"] — optional hints about expected approach
forbidden = []         # e.g. ["unsafe"] — things the solution must not use

[task.tests]
# Each test is a standalone Rust test function body.
# The test assumes the solution exposes the described public API.
[[task.tests.cases]]
name = "basic_put_get"
code = """
let mut cache = LruCache::new(2);
cache.put(1, 1);
cache.put(2, 2);
assert_eq!(cache.get(&1), Some(&1));
"""

[[task.tests.cases]]
name = "eviction"
code = """
let mut cache = LruCache::new(2);
cache.put(1, 1);
cache.put(2, 2);
cache.put(3, 3);
assert_eq!(cache.get(&1), None);
assert_eq!(cache.get(&3), Some(&3));
"""
```

Rules for task generation:

- All tasks target **Rust** as the implementation language.
- Each task must have **at least 4 test cases**, including edge cases.
- Tests must be self-contained: they should compile and run given only the solution code.
- Do not generate trivially similar tasks (e.g., don’t have both “reverse string” and “reverse array”).
- Spread across different CS domains: data structures, algorithms, string processing, concurrency, parsing, networking, math.

### Reference Solutions

For each task, create a directory `tasks/solutions/{task_id}/` containing:

- `lib.rs` — the canonical solution
- `tests.rs` — the test cases from the TOML, wrapped in a proper `#[cfg(test)]` module

These are used by the eval harness to validate that tests are correct.

-----

## 3. Prompt Templates

Create three template files in `prompts/templates/`:

### `personified.md`

```markdown
Hey, I've got a coding challenge I'd love your take on.

{{task_description}}

What do you think would be the best approach here? Could you write a Rust implementation for this? I'm curious how you'd handle the edge cases too.

{{constraints_block}}
```

### `polite_directive.md`

```markdown
Please implement the following in Rust.

{{task_description}}

Please ensure your implementation handles all edge cases. Please write clean, idiomatic Rust code.

{{constraints_block}}
```

### `bare_directive.md`

```markdown
Implement the following in Rust.

{{task_description}}

Handle all edge cases. Write clean, idiomatic Rust code.

{{constraints_block}}
```

The `{{constraints_block}}` placeholder resolves to:

- If `must_use` is non-empty: `Required: use {{must_use}}.`
- If `forbidden` is non-empty: `Do not use: {{forbidden}}.`
- Otherwise: empty string.

Also create `prompts/templates/system.md`:

```markdown
You are a Rust developer. Respond with only the Rust source code. Do not include explanations, markdown fences, or commentary. Output valid, compilable Rust code only.
```

-----

## 4. Runner (Rust)

Create a Rust binary in `runner/` that:

1. Reads all task TOML files from `tasks/definitions/`.
1. For each task × each prompt style × N repetitions (configurable, default 3):
- Renders the prompt from the template.
- Calls the Anthropic Messages API (`claude-sonnet-4-5-20250929` by default, configurable via `--model`).
- Saves the raw response to `results/{task_id}/{style}/{run_n}.rs`.
- Saves metadata (latency, token counts, model, timestamp) to `results/{task_id}/{style}/{run_n}.meta.json`.
1. Respects rate limits with exponential backoff.
1. Supports `--dry-run` to print rendered prompts without calling the API.
1. Supports `--tasks` filter (glob pattern on task ID).
1. Supports `--styles` filter (comma-separated list of style names).
1. Reads `ANTHROPIC_API_KEY` from environment.

Dependencies: `reqwest`, `tokio`, `serde`, `serde_json`, `toml`, `clap`, `tracing`.

Use a simple template engine — just string replacement of `{{task_description}}` and `{{constraints_block}}`. No need for a full template library.

-----

## 5. Evaluator (Rust)

Create a Rust binary in `eval/` that:

1. Iterates over all result files in `results/`.
1. For each `.rs` file:
- Extracts the code (strips any markdown fences if present despite instructions).
- Combines it with the test cases from the corresponding task TOML.
- Writes a temp crate, runs `cargo test --release` with a timeout.
- Records: pass/fail per test, total pass rate, compile success, stderr output.
1. Also computes static metrics on the code:
- Lines of code (excluding blanks/comments).
- Number of `unsafe` blocks.
- Number of `unwrap()`/`expect()` calls.
- Whether it uses the `must_use` items (if specified).
- Whether it uses `forbidden` items (if specified).
1. Outputs a consolidated `eval/results.json`:

```json
{
  "task_id": "3_01_lru_cache",
  "style": "personified",
  "run": 1,
  "compiled": true,
  "tests_total": 4,
  "tests_passed": 3,
  "tests_failed": ["eviction"],
  "loc": 42,
  "unsafe_count": 0,
  "unwrap_count": 2,
  "must_use_satisfied": true,
  "forbidden_violated": false,
  "latency_ms": 3200,
  "input_tokens": 450,
  "output_tokens": 1200
}
```

-----

## 6. Analysis Script (Python via uv)

Create `analysis/pyproject.toml`:

```toml
[project]
name = "prompting-style-analysis"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = [
    "pandas>=2.2",
    "scipy>=1.14",
    "matplotlib>=3.9",
]
```

Create `analysis/analyze.py` that:

1. Loads `eval/results.json`.
1. Computes per-style aggregate metrics:
- Test pass rate (mean, std, CI).
- Compilation success rate.
- Average LOC.
- Average latency and token usage.
1. Runs statistical tests:
- Kruskal-Wallis test across all three styles.
- Pairwise Mann-Whitney U tests with Bonferroni correction.
- Effect sizes (Cliff’s delta).
1. Stratifies results by difficulty tier.
1. Outputs:
- `analysis/summary.md` — markdown table of results.
- `analysis/figures/` — bar charts and box plots (pass rate by style, pass rate by style×tier).

Run via: `cd analysis && uv run python analyze.py`

-----

## 7. README.md

Generate a README that covers:

- **Purpose**: one-paragraph description of the study.
- **Hypothesis**: null hypothesis is that prompting style has no effect on code quality.
- **Setup**: prerequisites (Rust toolchain, uv, API key).
- **Usage**:
  - `cd runner && cargo run -- --dry-run` to preview prompts.
  - `cd runner && cargo run` to execute the experiment.
  - `cd eval && cargo run` to evaluate responses.
  - `cd analysis && uv run python analyze.py` to analyze.
- **Configuration**: how to change model, repetition count, task filters.
- **Limitations**: list known threats to validity (single model, English only, Rust only, synthetic tasks, system prompt anchoring, temperature effects).
- **License**: MIT.

-----

## 8. CI Integration

Create `.github/workflows/generate.yml`:

```yaml
name: Generate Study Data

on:
  workflow_dispatch:
    inputs:
      model:
        description: 'Model to test against'
        default: 'claude-sonnet-4-5-20250929'
        type: string
      repetitions:
        description: 'Runs per task per style'
        default: '3'
        type: string

env:
  ANTHROPIC_API_KEY: ${{ secrets.ANTHROPIC_API_KEY }}

jobs:
  run-experiment:
    runs-on: ubuntu-latest
    timeout-minutes: 120
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install uv
        uses: astral-sh/setup-uv@v4

      - name: Run experiment
        run: |
          cd runner
          cargo run --release -- \
            --model "${{ inputs.model }}" \
            --repetitions "${{ inputs.repetitions }}"

      - name: Evaluate results
        run: |
          cd eval
          cargo run --release

      - name: Analyze results
        run: |
          cd analysis
          uv run python analyze.py

      - name: Upload results
        uses: actions/upload-artifact@v4
        with:
          name: experiment-results-${{ github.run_id }}
          path: |
            results/
            eval/results.json
            analysis/summary.md
            analysis/figures/
```

-----

## Execution Checklist

After generating everything:

1. Verify all 50 task TOML files exist and parse correctly.
1. Verify all 50 reference solutions compile and pass their tests: `cd tasks/solutions/{id} && cargo test`.
1. Verify `runner` compiles: `cd runner && cargo check`.
1. Verify `eval` compiles: `cd eval && cargo check`.
1. Verify `analysis/analyze.py` runs without errors on a mock `eval/results.json` (generate a small synthetic one for testing).
1. Verify the GitHub Actions workflow YAML is valid.

If any check fails, fix the issue before proceeding to the next item.
