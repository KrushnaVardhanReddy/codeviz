TASK: T41 — Circular Dependency Detection

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement circular dependency detection in the CodeGraph and expose it via the CLI and a new MCP tool.

Files to Modify/Create:
- `codeviz-core/src/graph.rs` (add `find_import_cycles` method)
- `codeviz-cli/src/main.rs` (add `--detect-cycles` flag)
- `codeviz-mcp/src/tools.rs` (add `find_circular_dependencies` tool)

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/circular_deps.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- Implement a graph traversal algorithm (like DFS) to detect cycles.
- Only consider edges where `kind == EdgeKind::Imports`.
- Ensure `cargo clippy --all -- -D warnings` and `cargo test --all` pass.
- Write unit tests for `find_import_cycles`.


═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- Use stable Rust 2021 edition. Do NOT use unstable features like `let_chains`.
- Always run `cargo test --all` and `cargo clippy --all -- -D warnings` before completing the task.
