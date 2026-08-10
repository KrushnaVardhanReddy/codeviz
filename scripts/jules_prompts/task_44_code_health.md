TASK: T44 — Code Health Score

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement a composite Code Health Score (1–10) for every file in the `CodeGraph`,
computed purely from graph metrics (no external tools needed). Expose via CLI
(`--health`) and a new MCP tool (`get_code_health`).

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/code_health.md

═══════════════════════════════════════════════════════════════
BACKGROUND — WHAT ALREADY EXISTS
═══════════════════════════════════════════════════════════════

- `CodeGraph` is defined in `codeviz-core/src/graph.rs`.
- Circular dependency detection exists from T41 (`find_import_cycles`).

═══════════════════════════════════════════════════════════════
DELIVERABLES
═══════════════════════════════════════════════════════════════

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. MODIFY: codeviz-core/src/graph.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Implement the following:

```rust
/// Returns a map of file_path → health score (1.0 to 10.0).
pub fn compute_health_scores(&self) -> std::collections::HashMap<String, f64> {
    // For each unique file_path represented in the graph:
    // 1. Start with score = 10.0
    // 2. Deduct for high fan-out: count outgoing Imports edges. For each > 10: deduct 0.2.
    // 3. Deduct for high fan-in: count incoming Imports edges. For each > 15: deduct 0.15.
    // 4. Deduct for large file (proxy): count nodes with the same file_path. For each > 20: deduct 0.1.
    // 5. Deduct 3.0 if the file is part of any import cycle (call find_import_cycles()).
    // 6. Clamp final score to [1.0, 10.0].
}
```

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
2. MODIFY: codeviz-cli/src/main.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Add a `--health` flag to the `run` subcommand.
When enabled:
- Call `compute_health_scores()` after generating the graph.
- Print a sorted table to stdout, with the unhealthiest files first:
  ```
  HEALTH SCORE REPORT (sorted by worst first)
  ─────────────────────────────────────────────
  1.0   src/db/query.rs        ⚠️  (circular dep, high fan-in)
  4.2   src/auth/middleware.rs
  8.5   src/utils/logger.rs    ✅
  ```

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
3. MODIFY: codeviz-mcp/src/tools.rs & server.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Register a new MCP tool:
- Name: `get_code_health`
- Input: `{ "path": string }`
- Output: `{ "scores": [{ "file_path": string, "score": float }] }` sorted by score ascending.

═══════════════════════════════════════════════════════════════
CRITICAL IMPLEMENTATION RULES
═══════════════════════════════════════════════════════════════
1. The score MUST be clamped between 1.0 and 10.0. Never outside this range.
2. Reuse `find_import_cycles()` from T41. Do NOT re-implement cycle detection.
3. Write unit tests verifying that a highly coupled and cyclic file gets a low score.
4. Ensure `cargo test --all` and `cargo clippy --all -- -D warnings` pass.

Commit: "jules: T44 — Composite Code Health Score engine and MCP tool"
Target branch: feat-t44-code-health
