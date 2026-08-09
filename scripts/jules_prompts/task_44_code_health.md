TASK: T44 — Code Health Score

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement a Code Health Score (1-10) metric for files based on coupling and complexity, exposing it via CLI and MCP.

Files to Modify/Create:
- `codeviz-core/src/graph.rs` (add `compute_health_scores` method)
- `codeviz-cli/src/main.rs` (add `--health` flag)
- `codeviz-mcp/src/tools.rs` (add `get_code_health` tool)

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/code_health.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- The score should be calculated per `NodeKind::File`.
- Maximum score is 10.0, minimum is 1.0.
- Deduct points for high fan-in, high fan-out, high node count per file, and cycle participation (you may need to call `find_import_cycles`).
- Ensure `cargo clippy --all -- -D warnings` and `cargo test --all` pass.
- Write unit tests for `compute_health_scores`.
