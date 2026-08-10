TASK: T41 — Circular Dependency Detection

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement a circular dependency detection algorithm on the `CodeGraph` and expose
it via both the CLI (`--detect-cycles`) and a new MCP tool (`find_circular_dependencies`).
Circular import cycles are a major source of technical debt and must be clearly surfaced.

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/circular_deps.md

═══════════════════════════════════════════════════════════════
BACKGROUND — WHAT ALREADY EXISTS
═══════════════════════════════════════════════════════════════

- `CodeGraph` is defined in `codeviz-core/src/graph.rs` with `Node`, `Edge`, and `EdgeKind`.
- `EdgeKind::Imports` represents file-level import dependencies.
- The CLI entry point is `codeviz-cli/src/main.rs`.
- MCP tools are registered in `codeviz-mcp/src/tools.rs` and `server.rs`.

═══════════════════════════════════════════════════════════════
DELIVERABLES
═══════════════════════════════════════════════════════════════

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. MODIFY: codeviz-core/src/graph.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Add the following public method to `CodeGraph`:

```rust
/// Returns a list of import cycles. Each cycle is a Vec of Node IDs
/// representing the closed path, e.g. ["a.py", "b.py", "c.py", "a.py"].
pub fn find_import_cycles(&self) -> Vec<Vec<String>> {
    // Filter to EdgeKind::Imports only.
    // Implement Tarjan's SCC algorithm or DFS with a visited-stack.
    // Return each strongly connected component of size > 1 as a cycle path.
}
```

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
2. MODIFY: codeviz-core/src/render/mermaid.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
When cycles are found, add `linkStyle` directives to the Mermaid output to
color the cyclic edges in RED. Use `classDef cycle fill:#FF4444` for cycle nodes.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
3. MODIFY: codeviz-cli/src/main.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Add a `--detect-cycles` flag to the `run` subcommand.
When enabled:
- After generating the `CodeGraph`, call `find_import_cycles()`.
- If cycles exist, print each one to stderr (e.g., `⚠️ Cycle: a.py → b.py → c.py → a.py`) and exit with code `1`.
- If no cycles, print `✅ No circular dependencies found.` and exit `0`.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
4. MODIFY: codeviz-mcp/src/tools.rs & server.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Register a new MCP tool:
- Name: `find_circular_dependencies`
- Input: `{ "path": string }`
- Output: `{ "cycles": [ ["a.py", "b.py", "c.py", "a.py"], ... ], "count": number }`

═══════════════════════════════════════════════════════════════
CRITICAL IMPLEMENTATION RULES
═══════════════════════════════════════════════════════════════
1. Use only `std::collections` — do NOT add external graph crates.
2. No `unwrap()`. Return `Result` types.
3. Write unit tests: a graph with a known cycle A→B→C→A must be detected.
4. A graph with no cycles must return an empty vec.
5. Ensure `cargo test --all` and `cargo clippy --all -- -D warnings` pass.

Commit: "jules: T41 — Circular dependency detection engine and MCP tool"
Target branch: feat-t41-circular-deps
