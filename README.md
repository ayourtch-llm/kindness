# Kindness — LLM Prompting Style Study

Does the way you phrase a prompt to an LLM coding agent affect the quality of its output? This project is a controlled experiment that investigates three prompting styles — **personified** (conversational peer), **polite-directive** (instructions with "please"), and **bare-directive** (terse commands) — across 75 Rust coding tasks of varying difficulty (5 tiers × 15 tasks). The harness sends each task in each style to any OpenAI-compatible LLM endpoint, evaluates the generated code against test suites, and performs statistical analysis on the results.

## Hypothesis

**Null hypothesis (H₀):** Prompting style has no statistically significant effect on code quality metrics (test pass rate, compilation success, code size, use of unsafe constructs).

## Repository Structure

```
tasks/
  definitions/       # 75 task TOML files (15 per tier)
  solutions/         # Reference solutions (one dir per task)
prompts/
  templates/         # Three prompt style templates + system prompt
runner/
  src/main.rs        # Rust binary: API caller + response collector
  Cargo.toml
eval/
  src/main.rs        # Rust binary: test runner + metrics
  Cargo.toml
analysis/
  analyze.py         # Statistical analysis script (run via uv)
  pyproject.toml
results/             # Raw API responses (generated at experiment time)
.github/workflows/   # CI workflow for running experiments
```

## Prerequisites

- **Rust toolchain** (stable) — install via [rustup](https://rustup.rs/)
- **uv** — Python package manager, install via `curl -LsSf https://astral.sh/uv/install.sh | sh`
- **API key** for your chosen LLM endpoint (OpenAI, vllm, llama.cpp, etc.)

## Usage

### 1. Preview prompts (dry run)

```bash
cd runner && cargo run -- --dry-run
```

### 2. Run the experiment

The runner uses an **OpenAI-compatible Chat Completions API**, so it works with OpenAI, vllm, llama.cpp, or any compatible endpoint.

```bash
# Set your API key (provider-agnostic name)
export LLM_API_KEY="your-api-key-here"

# OpenAI
cd runner && cargo run --release -- --model gpt-4o

# Local vllm instance
cd runner && cargo run --release -- \
  --api-url http://localhost:8000/v1/chat/completions \
  --model meta-llama/Llama-3-8B

# Local llama.cpp
cd runner && cargo run --release -- \
  --api-url http://localhost:8080/v1/chat/completions \
  --model local-model

# Anthropic (via OpenAI-compatible endpoint)
cd runner && cargo run --release -- \
  --api-url https://api.anthropic.com/v1/messages \
  --model claude-sonnet-4-5-20250929
```

### 3. Evaluate responses

```bash
cd eval && cargo run --release
```

### 4. Analyze results

```bash
cd analysis && uv run python analyze.py
```

## Configuration

| Flag | Default | Description |
|------|---------|-------------|
| `--api-url` | `https://api.openai.com/v1/chat/completions` | OpenAI-compatible endpoint URL |
| `--model` | `gpt-4o` | Model name to request |
| `--repetitions` | `3` | Runs per task × style combination |
| `--tasks` | all | Glob filter on task ID (e.g. `"3_*"` for tier 3 only) |
| `--styles` | all | Comma-separated style filter (e.g. `"personified,bare_directive"`) |
| `--max-tokens` | `4096` | Max tokens for completion |
| `--temperature` | `0.0` | Sampling temperature (0 for reproducibility) |
| `--dry-run` | off | Print rendered prompts without calling the API |

## Task Tiers

| Tier | Label | Count | Description | Examples |
|------|-------|-------|-------------|----------|
| 1 | Trivial | 15 | Single function, minimal logic | FizzBuzz, palindrome check |
| 2 | Easy | 15 | Single function with edge cases | Binary search, Roman numerals |
| 3 | Medium | 15 | Multiple functions / data structures | LRU cache, trie, JSON parser |
| 4 | Hard | 15 | Algorithmic complexity, system components | Red-black tree, A* pathfinding |
| 5 | Expert | 15 | Multi-component, architectural decisions | HTTP router, regex engine, type checker |

## Limitations

- **Single model per run** — cross-model comparison requires separate runs.
- **English only** — prompts and tasks are all in English.
- **Rust only** — tasks target Rust; results may not generalize to other languages.
- **Synthetic tasks** — real-world coding involves more context and ambiguity.
- **System prompt anchoring** — the system prompt instructs "code only" output, which may reduce style effects.
- **Temperature effects** — default temperature 0.0 maximizes determinism but limits exploration of the output distribution.
- **No human evaluation** — only automated test pass rates and static metrics are measured.

## License

MIT
