TASK: T42 — Unused Module Detection

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement unused module detection in the CodeGraph and expose it via the CLI and a new MCP tool.

Files to Modify/Create:
- `codeviz-core/src/graph.rs` (add `find_unused_modules` method)
- `codeviz-cli/src/main.rs` (add `--find-unused` flag)
- `codeviz-mcp/src/tools.rs` (add `find_unused_modules` tool)

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/unused_modules.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- Iterate through `NodeKind::File` nodes and find those with no incoming `EdgeKind::Imports`.
- Ensure `cargo clippy --all -- -D warnings` and `cargo test --all` pass.
- Write unit tests for `find_unused_modules`.


═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- Use stable Rust 2021 edition. Do NOT use unstable features like `let_chains`.
- Always run `cargo test --all` and `cargo clippy --all -- -D warnings` before completing the task.
