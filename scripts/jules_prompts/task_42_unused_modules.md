TASK: T42 — Unused Module Detection

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement dead code / unused module detection on the `CodeGraph`. A module is
considered unused if it has no incoming `Imports` edges and is not an entry point.
Expose this via CLI (`--find-unused`) and a new MCP tool (`find_unused_modules`).

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/unused_modules.md

═══════════════════════════════════════════════════════════════
BACKGROUND — WHAT ALREADY EXISTS
═══════════════════════════════════════════════════════════════

- `CodeGraph` is defined in `codeviz-core/src/graph.rs` with `Node`, `Edge`, and `EdgeKind`.
- `NodeKind::File` represents file-level nodes.
- `EdgeKind::Imports` represents file-level import dependencies.
- Entry points (like `main.rs`, `index.js`) may be explicitly tracked in the config or inferred by name.

═══════════════════════════════════════════════════════════════
DELIVERABLES
═══════════════════════════════════════════════════════════════

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. MODIFY: codeviz-core/src/graph.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Add the following public method to `CodeGraph`:

```rust
/// Returns a list of Node IDs (for NodeKind::File nodes only)
/// that have zero incoming EdgeKind::Imports edges.
pub fn find_unused_modules(&self) -> Vec<String> {
    // Collect all node IDs that appear as the target of at least one Imports edge.
    // Then filter all File nodes to those that are NOT in that set.
    // Exclude common entry point file names: main.rs, index.js, index.ts, __init__.py.
}
```

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
2. MODIFY: codeviz-cli/src/main.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Add a `--find-unused` flag to the `run` subcommand.
When enabled:
- Call `find_unused_modules()` after generating the graph.
- Print each unused module file path to stdout.
- If none, print `✅ No unused modules detected.`

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
3. MODIFY: codeviz-mcp/src/tools.rs & server.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Register a new MCP tool:
- Name: `find_unused_modules`
- Input: `{ "path": string }`
- Output: `{ "unused_modules": [ { "node_id": string, "file_path": string } ], "count": number }`

═══════════════════════════════════════════════════════════════
CRITICAL IMPLEMENTATION RULES
═══════════════════════════════════════════════════════════════
1. The algorithm must operate only on `NodeKind::File` nodes.
2. Entry points (`main.rs`, `index.js`, etc.) must NOT be flagged as unused.
3. Write unit tests: a disconnected node that is not an entry point must be found;
   `main.rs` with no incoming edges must NOT be flagged.
4. Ensure `cargo test --all` and `cargo clippy --all -- -D warnings` pass.

Commit: "jules: T42 — Unused module detection engine and MCP tool"
Target branch: feat-t42-unused-modules
