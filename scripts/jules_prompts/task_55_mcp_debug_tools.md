TASK: T55 — MCP Debugging Tools

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Add three new powerful graph traversal tools to the MCP server: `trace_call_path`,
`get_callers_recursive`, and `get_blast_radius`. These tools enable AI agents
to perform deep architectural debugging without needing full codebase context.

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/mcp_debug_tools.md

═══════════════════════════════════════════════════════════════
BACKGROUND — WHAT ALREADY EXISTS
═══════════════════════════════════════════════════════════════

EXISTING INFRASTRUCTURE:
  - The `CodeGraph` structure is defined in `codeviz-core/src/graph.rs`.
  - MCP server implementation is in `codeviz-mcp/src/`.
  - Existing MCP tools are defined and registered in `codeviz-mcp/src/tools.rs` and `server.rs`.

═══════════════════════════════════════════════════════════════
DELIVERABLES
═══════════════════════════════════════════════════════════════

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. MODIFY: codeviz-core/src/graph.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Add three new public methods to the `CodeGraph` struct. These methods must be
pure graph algorithms that operate on `self.nodes` and `self.edges`.

  /// Returns all paths (up to `max_paths`) from `start_node_id` to `target_node_id`.
  /// Paths follow the `Calls` edge kind.
  pub fn all_paths(&self, start_node_id: &str, target_node_id: &str, max_paths: usize) -> Vec<Vec<String>> {
      // Implement DFS with backtracking.
      // Short-circuit if max_paths is reached.
  }

  /// Returns the recursive caller tree up to `max_depth`.
  pub fn callers_recursive(&self, target_node_id: &str, max_depth: usize) -> serde_json::Value {
      // Implement reverse graph traversal.
      // Detect cycles to avoid infinite loops.
  }

  /// Returns all transitively reachable nodes from the given node.
  pub fn blast_radius(&self, start_node_id: &str) -> Vec<String> {
      // Implement forward BFS/DFS.
      // Detect cycles.
  }

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
2. MODIFY: codeviz-mcp/src/tools.rs & codeviz-mcp/src/server.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Register three new tools in the MCP server:

1. `trace_call_path`
   - Inputs: `from` (string), `to` (string), `path` (string, directory), `max_paths` (number, default 10, max 50).
2. `get_callers_recursive`
   - Inputs: `fn_name` (string, ID), `path` (string, directory), `depth` (number, default 3, max 10).
3. `get_blast_radius`
   - Inputs: `fn_name` (string, ID), `path` (string, directory).

Ensure the `tools/list` endpoint correctly reports their JSON schemas.
Update the `tools/call` handler to execute the respective `CodeGraph` methods.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
3. CREATE/MODIFY: Unit Tests
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Add unit tests in `codeviz-core/src/graph.rs` (or a dedicated test file) to
validate:
- `all_paths` finds all linear paths and respects `max_paths`.
- `callers_recursive` handles cycles gracefully and respects `max_depth`.
- `blast_radius` handles cycles gracefully.

═══════════════════════════════════════════════════════════════
CRITICAL IMPLEMENTATION RULES
═══════════════════════════════════════════════════════════════
1. Do NOT use `unwrap()`. Return standard `Result` types.
2. All graph traversal methods MUST detect cycles and short-circuit to prevent infinite loops.
3. Use only the Rust standard library for graph algorithms. No external graph crates.
4. Ensure `cargo test --all` and `cargo clippy --all -- -D warnings` pass.

Commit: "jules: T55 — MCP Debugging Tools"
Target branch: feat-t55-mcp-debug-tools
