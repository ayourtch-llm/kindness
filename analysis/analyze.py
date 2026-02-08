"""Analyze prompting-style experiment results.

Usage:
    cd analysis && uv run python analyze.py [--results PATH]
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from itertools import combinations
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
from scipy import stats


# ---------------------------------------------------------------------------
# Cliff's delta
# ---------------------------------------------------------------------------

def cliffs_delta(a: np.ndarray, b: np.ndarray) -> tuple[float, str]:
    """Compute Cliff's delta effect size for two samples.

    Returns (delta, magnitude) where magnitude is one of
    negligible / small / medium / large.
    """
    a, b = np.asarray(a, dtype=float), np.asarray(b, dtype=float)
    if len(a) == 0 or len(b) == 0:
        return 0.0, "negligible"
    # All pairwise comparisons via broadcasting
    diff = a[:, None] - b[None, :]
    more = np.sum(diff > 0)
    less = np.sum(diff < 0)
    n = len(a) * len(b)
    delta = (more - less) / n

    abs_d = abs(delta)
    if abs_d < 0.147:
        mag = "negligible"
    elif abs_d < 0.33:
        mag = "small"
    elif abs_d < 0.474:
        mag = "medium"
    else:
        mag = "large"
    return delta, mag


# ---------------------------------------------------------------------------
# Data loading
# ---------------------------------------------------------------------------

def load_results(path: Path) -> pd.DataFrame:
    """Load results.json into a DataFrame, or exit gracefully."""
    if not path.exists():
        print(f"Results file not found: {path}", file=sys.stderr)
        sys.exit(1)

    text = path.read_text()
    if not text.strip():
        print(f"Results file is empty: {path}", file=sys.stderr)
        sys.exit(1)

    data = json.loads(text)
    if not data:
        print("Results JSON contains no records.", file=sys.stderr)
        sys.exit(1)

    df = pd.DataFrame(data)
    return df


# ---------------------------------------------------------------------------
# Aggregate metrics
# ---------------------------------------------------------------------------

def compute_aggregates(df: pd.DataFrame) -> pd.DataFrame:
    """Compute per-style aggregate metrics."""
    rows: list[dict] = []
    for style, g in df.groupby("style"):
        n = len(g)
        pass_rate = g["pass"].mean()
        pass_std = g["pass"].std(ddof=1) if n > 1 else 0.0
        ci95 = 1.96 * pass_std / np.sqrt(n) if n > 1 else 0.0

        compile_rate = g["compiles"].mean() if "compiles" in g.columns else np.nan
        avg_loc = g["loc"].mean() if "loc" in g.columns else np.nan
        avg_latency = g["latency_s"].mean() if "latency_s" in g.columns else np.nan
        avg_tokens = g["total_tokens"].mean() if "total_tokens" in g.columns else np.nan

        rows.append({
            "style": style,
            "n": n,
            "pass_rate": pass_rate,
            "pass_std": pass_std,
            "pass_ci95": ci95,
            "compile_rate": compile_rate,
            "avg_loc": avg_loc,
            "avg_latency_s": avg_latency,
            "avg_tokens": avg_tokens,
        })
    return pd.DataFrame(rows)


def compute_tier_aggregates(df: pd.DataFrame) -> pd.DataFrame:
    """Compute per-style×tier aggregate metrics."""
    rows: list[dict] = []
    for (style, tier), g in df.groupby(["style", "tier"]):
        n = len(g)
        pass_rate = g["pass"].mean()
        pass_std = g["pass"].std(ddof=1) if n > 1 else 0.0
        ci95 = 1.96 * pass_std / np.sqrt(n) if n > 1 else 0.0
        rows.append({
            "style": style,
            "tier": tier,
            "n": n,
            "pass_rate": pass_rate,
            "pass_std": pass_std,
            "pass_ci95": ci95,
        })
    return pd.DataFrame(rows)


# ---------------------------------------------------------------------------
# Failure-mode diagnostics
# ---------------------------------------------------------------------------

def diagnose_failures(df: pd.DataFrame) -> dict:
    """Investigate failure modes across tiers and styles.

    Returns a dict with diagnostic findings for inclusion in the report.
    All analysis is purely algorithmic / local – no LLM calls.
    """
    diag: dict = {}

    # --- 1. Compilation-failure vs. test-failure breakdown by tier ----------
    if "tier" in df.columns and "compiles" in df.columns:
        tier_failure_rows: list[dict] = []
        for tier, g in df.groupby("tier"):
            n = len(g)
            compile_fails = int((g["compiles"] == 0).sum())
            compile_ok = n - compile_fails
            # Among those that compiled, how many had at least one test fail?
            compiled_mask = g["compiles"] == 1
            test_failures = int((g.loc[compiled_mask, "pass"] < 1.0).sum()) if compiled_mask.any() else 0
            full_pass = int((g.loc[compiled_mask, "pass"] == 1.0).sum()) if compiled_mask.any() else 0
            tier_failure_rows.append({
                "tier": int(tier),
                "n": n,
                "compile_failures": compile_fails,
                "test_failures": test_failures,
                "full_pass": full_pass,
                "compile_rate": compile_ok / n if n else 0.0,
                "pass_rate_if_compiled": full_pass / compile_ok if compile_ok else 0.0,
            })
        diag["tier_failure_breakdown"] = tier_failure_rows

    # --- 2. Markdown-fence detection correlation ---------------------------
    if "had_markdown_fences" in df.columns:
        md_rows: list[dict] = []
        for label, mask in [("with_fences", df["had_markdown_fences"]),
                            ("without_fences", ~df["had_markdown_fences"])]:
            sub = df.loc[mask]
            if sub.empty:
                continue
            md_rows.append({
                "group": label,
                "n": len(sub),
                "compile_rate": sub["compiles"].mean() if "compiles" in sub.columns else np.nan,
                "pass_rate": sub["pass"].mean(),
            })
        diag["markdown_fences"] = md_rows

        # Per-tier markdown fence breakdown
        if "tier" in df.columns:
            md_tier_rows: list[dict] = []
            for tier, g in df.groupby("tier"):
                fenced = g["had_markdown_fences"].sum()
                md_tier_rows.append({
                    "tier": int(tier),
                    "n": len(g),
                    "fenced_count": int(fenced),
                    "fenced_pct": fenced / len(g) if len(g) else 0.0,
                })
            diag["markdown_fences_by_tier"] = md_tier_rows

    # --- 3. Tier anomaly detection -----------------------------------------
    # Flag any tier whose pass-rate is lower than all higher-numbered tiers.
    if "tier" in df.columns:
        tier_rates = df.groupby("tier")["pass"].mean().sort_index()
        anomalies: list[dict] = []
        tiers = list(tier_rates.index)
        for i, t in enumerate(tiers):
            higher = [tier_rates[h] for h in tiers if h > t]
            if higher and tier_rates[t] < min(higher):
                anomalies.append({
                    "tier": int(t),
                    "pass_rate": float(tier_rates[t]),
                    "min_higher_tier_rate": float(min(higher)),
                })
        diag["tier_anomalies"] = anomalies

    # --- 4. Per-task failure counts (top N worst tasks) --------------------
    if "task_id" in df.columns:
        task_stats = (
            df.groupby("task_id")
            .agg(
                n=("pass", "size"),
                pass_rate=("pass", "mean"),
                compile_rate=("compiles", "mean") if "compiles" in df.columns else ("pass", "size"),
            )
            .sort_values("pass_rate")
        )
        worst = task_stats.head(10).reset_index()
        diag["worst_tasks"] = worst.to_dict(orient="records")

    return diag


# ---------------------------------------------------------------------------
# Statistical tests
# ---------------------------------------------------------------------------

def run_statistical_tests(df: pd.DataFrame, styles: list[str]) -> dict:
    """Run Kruskal-Wallis and pairwise Mann-Whitney U tests."""
    groups = [df.loc[df["style"] == s, "pass"].values for s in styles]

    # Kruskal-Wallis across all styles
    if len(groups) >= 2 and all(len(g) > 0 for g in groups):
        kw_stat, kw_p = stats.kruskal(*groups)
    else:
        kw_stat, kw_p = np.nan, np.nan

    # Pairwise Mann-Whitney U with Bonferroni correction
    pairs = list(combinations(range(len(styles)), 2))
    n_comparisons = len(pairs) if pairs else 1
    pairwise: list[dict] = []
    for i, j in pairs:
        a, b = groups[i], groups[j]
        if len(a) == 0 or len(b) == 0:
            continue
        u_stat, u_p = stats.mannwhitneyu(a, b, alternative="two-sided")
        p_corrected = min(u_p * n_comparisons, 1.0)
        delta, mag = cliffs_delta(a, b)
        pairwise.append({
            "comparison": f"{styles[i]} vs {styles[j]}",
            "U": u_stat,
            "p": u_p,
            "p_corrected": p_corrected,
            "cliffs_delta": delta,
            "effect_size": mag,
        })

    return {
        "kruskal_wallis": {"H": kw_stat, "p": kw_p},
        "pairwise": pairwise,
    }


# ---------------------------------------------------------------------------
# Markdown report
# ---------------------------------------------------------------------------

def generate_summary(
    agg: pd.DataFrame,
    tier_agg: pd.DataFrame,
    test_results: dict,
    diagnostics: dict,
    out_path: Path,
) -> None:
    """Write summary.md."""
    lines: list[str] = []

    lines.append("# Prompting-Style Analysis Summary\n")

    # Overall metrics table
    lines.append("## Overall Metrics by Style\n")
    lines.append("| Style | n | Pass Rate | Std | 95% CI | Compile Rate | Avg LOC | Avg Latency (s) | Avg Tokens |")
    lines.append("|-------|---|-----------|-----|--------|--------------|---------|-----------------|------------|")
    for _, r in agg.iterrows():
        lines.append(
            f"| {r['style']} | {r['n']:.0f} | {r['pass_rate']:.3f} | {r['pass_std']:.3f} "
            f"| ±{r['pass_ci95']:.3f} | {r['compile_rate']:.3f} | {r['avg_loc']:.1f} "
            f"| {r['avg_latency_s']:.2f} | {r['avg_tokens']:.0f} |"
        )
    lines.append("")

    # Statistical tests
    kw = test_results["kruskal_wallis"]
    lines.append("## Statistical Tests\n")
    lines.append(f"**Kruskal-Wallis:** H = {kw['H']:.4f}, p = {kw['p']:.6f}\n")

    if test_results["pairwise"]:
        lines.append("### Pairwise Mann-Whitney U (Bonferroni-corrected)\n")
        lines.append("| Comparison | U | p (raw) | p (corrected) | Cliff's δ | Effect Size |")
        lines.append("|------------|---|---------|---------------|-----------|-------------|")
        for pw in test_results["pairwise"]:
            lines.append(
                f"| {pw['comparison']} | {pw['U']:.1f} | {pw['p']:.6f} "
                f"| {pw['p_corrected']:.6f} | {pw['cliffs_delta']:.4f} | {pw['effect_size']} |"
            )
        lines.append("")

    # Per-tier breakdown
    lines.append("## Per-Tier Breakdown\n")
    lines.append("| Style | Tier | n | Pass Rate | Std | 95% CI |")
    lines.append("|-------|------|---|-----------|-----|--------|")
    for _, r in tier_agg.sort_values(["tier", "style"]).iterrows():
        lines.append(
            f"| {r['style']} | {r['tier']:.0f} | {r['n']:.0f} "
            f"| {r['pass_rate']:.3f} | {r['pass_std']:.3f} | ±{r['pass_ci95']:.3f} |"
        )
    lines.append("")

    # ── Failure-mode diagnostics ──────────────────────────────────────
    lines.append("## Failure-Mode Diagnostics\n")

    # Tier anomalies
    anomalies = diagnostics.get("tier_anomalies", [])
    if anomalies:
        lines.append("### Tier Anomalies\n")
        lines.append("The following tiers have a pass rate **lower** than all higher-difficulty tiers:\n")
        for a in anomalies:
            lines.append(
                f"- **Tier {a['tier']}**: pass rate {a['pass_rate']:.3f} "
                f"(lowest higher-tier rate: {a['min_higher_tier_rate']:.3f})"
            )
        lines.append("")
    else:
        lines.append("### Tier Anomalies\n")
        lines.append("No anomalies detected – pass rates decrease monotonically with tier difficulty.\n")

    # Compilation vs test-failure breakdown by tier
    breakdown = diagnostics.get("tier_failure_breakdown", [])
    if breakdown:
        lines.append("### Failure Breakdown by Tier\n")
        lines.append("| Tier | n | Compile Failures | Test Failures | Full Pass | Compile Rate | Pass Rate (if compiled) |")
        lines.append("|------|---|-----------------|---------------|-----------|--------------|------------------------|")
        for r in breakdown:
            lines.append(
                f"| {r['tier']} | {r['n']} | {r['compile_failures']} "
                f"| {r['test_failures']} | {r['full_pass']} "
                f"| {r['compile_rate']:.3f} | {r['pass_rate_if_compiled']:.3f} |"
            )
        lines.append("")

    # Markdown-fence impact
    md = diagnostics.get("markdown_fences", [])
    if md:
        lines.append("### Markdown Fence Impact\n")
        lines.append("Some LLM responses wrap code in markdown fences (` ```rust ... ``` `). "
                      "The eval harness strips these, but text outside fences (e.g. explanations) "
                      "previously leaked through and caused compilation failures.\n")
        lines.append("| Group | n | Compile Rate | Pass Rate |")
        lines.append("|-------|---|--------------|-----------|")
        for r in md:
            lines.append(
                f"| {r['group']} | {r['n']} | {r['compile_rate']:.3f} | {r['pass_rate']:.3f} |"
            )
        lines.append("")

    md_tier = diagnostics.get("markdown_fences_by_tier", [])
    if md_tier:
        lines.append("#### Markdown Fences by Tier\n")
        lines.append("| Tier | n | Fenced Count | Fenced % |")
        lines.append("|------|---|--------------|----------|")
        for r in md_tier:
            lines.append(
                f"| {r['tier']} | {r['n']} | {r['fenced_count']} | {r['fenced_pct']:.1%} |"
            )
        lines.append("")

    # Worst tasks
    worst = diagnostics.get("worst_tasks", [])
    if worst:
        lines.append("### Worst-Performing Tasks (up to 10)\n")
        lines.append("| Task ID | n | Pass Rate | Compile Rate |")
        lines.append("|---------|---|-----------|--------------|")
        for r in worst:
            lines.append(
                f"| {r['task_id']} | {r['n']} "
                f"| {r['pass_rate']:.3f} | {r.get('compile_rate', 0):.3f} |"
            )
        lines.append("")

    out_path.write_text("\n".join(lines))
    print(f"Wrote {out_path}")


# ---------------------------------------------------------------------------
# Figures
# ---------------------------------------------------------------------------

def generate_figures(df: pd.DataFrame, agg: pd.DataFrame, fig_dir: Path) -> None:
    """Generate bar charts and box plots."""
    fig_dir.mkdir(parents=True, exist_ok=True)
    styles = agg["style"].tolist()

    # 1. Bar chart – pass rate by style
    fig, ax = plt.subplots(figsize=(6, 4))
    x = np.arange(len(styles))
    ax.bar(x, agg["pass_rate"], yerr=agg["pass_ci95"], capsize=4, color=["#4c72b0", "#dd8452", "#55a868"][:len(styles)])
    ax.set_xticks(x)
    ax.set_xticklabels(styles)
    ax.set_ylabel("Pass Rate")
    ax.set_title("Test Pass Rate by Prompting Style")
    ax.set_ylim(0, 1)
    fig.tight_layout()
    fig.savefig(fig_dir / "pass_rate_by_style.png", dpi=150)
    plt.close(fig)
    print(f"Wrote {fig_dir / 'pass_rate_by_style.png'}")

    # 2. Box plot – pass distribution by style
    fig, ax = plt.subplots(figsize=(6, 4))
    data_by_style = [df.loc[df["style"] == s, "pass"].values for s in styles]
    bp = ax.boxplot(data_by_style, tick_labels=styles, patch_artist=True)
    colors = ["#4c72b0", "#dd8452", "#55a868"][:len(styles)]
    for patch, color in zip(bp["boxes"], colors):
        patch.set_facecolor(color)
        patch.set_alpha(0.7)
    ax.set_ylabel("Pass (0/1)")
    ax.set_title("Pass Distribution by Style")
    fig.tight_layout()
    fig.savefig(fig_dir / "pass_boxplot_by_style.png", dpi=150)
    plt.close(fig)
    print(f"Wrote {fig_dir / 'pass_boxplot_by_style.png'}")

    # 3. Grouped bar chart – pass rate by style × tier
    if "tier" in df.columns:
        tiers = sorted(df["tier"].unique())
        fig, ax = plt.subplots(figsize=(8, 5))
        width = 0.8 / len(styles)
        for idx, style in enumerate(styles):
            rates = []
            for tier in tiers:
                subset = df[(df["style"] == style) & (df["tier"] == tier)]
                rates.append(subset["pass"].mean() if len(subset) > 0 else 0.0)
            positions = np.arange(len(tiers)) + idx * width
            ax.bar(positions, rates, width, label=style,
                   color=["#4c72b0", "#dd8452", "#55a868"][idx % 3])
        ax.set_xticks(np.arange(len(tiers)) + width * (len(styles) - 1) / 2)
        ax.set_xticklabels([str(t) for t in tiers])
        ax.set_xlabel("Difficulty Tier")
        ax.set_ylabel("Pass Rate")
        ax.set_title("Pass Rate by Style × Difficulty Tier")
        ax.set_ylim(0, 1)
        ax.legend()
        fig.tight_layout()
        fig.savefig(fig_dir / "pass_rate_by_style_tier.png", dpi=150)
        plt.close(fig)
        print(f"Wrote {fig_dir / 'pass_rate_by_style_tier.png'}")


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main() -> None:
    parser = argparse.ArgumentParser(description="Analyze prompting-style experiment results.")
    parser.add_argument(
        "--results",
        type=Path,
        default=Path(__file__).resolve().parent.parent / "eval" / "results.json",
        help="Path to results.json (default: ../eval/results.json)",
    )
    args = parser.parse_args()

    df = load_results(args.results)

    # Derive columns from eval output format:
    # eval produces: compiled, tests_passed, tests_total, latency_ms, input_tokens, output_tokens
    # We need: pass (rate 0-1), compiles (0/1), tier, latency_s, total_tokens

    # Pass rate per result
    if "pass" not in df.columns:
        if "tests_passed" in df.columns and "tests_total" in df.columns:
            df["pass"] = (df["tests_passed"] / df["tests_total"].replace(0, np.nan)).fillna(0.0)
        else:
            df["pass"] = 0.0

    # Compile flag
    if "compiles" not in df.columns and "compiled" in df.columns:
        df["compiles"] = df["compiled"].astype(int)
    elif "compiles" in df.columns:
        df["compiles"] = df["compiles"].astype(int)

    # Derive tier from task_id (first character)
    if "tier" not in df.columns and "task_id" in df.columns:
        df["tier"] = df["task_id"].str[0].astype(int)

    # Latency in seconds
    if "latency_s" not in df.columns and "latency_ms" in df.columns:
        df["latency_s"] = df["latency_ms"] / 1000.0

    # Total tokens
    if "total_tokens" not in df.columns:
        if "input_tokens" in df.columns and "output_tokens" in df.columns:
            df["total_tokens"] = df["input_tokens"] + df["output_tokens"]

    df["pass"] = df["pass"].astype(float)

    styles = sorted(df["style"].unique().tolist())
    print(f"Loaded {len(df)} results across styles: {styles}")

    agg = compute_aggregates(df)
    tier_agg = compute_tier_aggregates(df) if "tier" in df.columns else pd.DataFrame()

    test_results = run_statistical_tests(df, styles)
    diagnostics = diagnose_failures(df)

    analysis_dir = Path(__file__).resolve().parent
    generate_summary(agg, tier_agg, test_results, diagnostics, analysis_dir / "summary.md")
    generate_figures(df, agg, analysis_dir / "figures")

    print("Analysis complete.")


if __name__ == "__main__":
    main()
