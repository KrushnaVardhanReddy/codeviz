TASK: T43 — PageRank & Centrality Scoring

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement PageRank algorithm to score code module centrality, exposing it via the CLI and a new MCP tool.

Files to Modify/Create:
- `codeviz-core/src/graph.rs` (add `compute_pagerank` method)
- `codeviz-cli/src/main.rs` (add `--pagerank` flag)
- `codeviz-mcp/src/tools.rs` (add `get_critical_modules` tool)

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/pagerank.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- Implement standard iterative PageRank on reversed `EdgeKind::Imports` edges.
- Ensure `cargo clippy --all -- -D warnings` and `cargo test --all` pass.
- Write unit tests for `compute_pagerank`.
