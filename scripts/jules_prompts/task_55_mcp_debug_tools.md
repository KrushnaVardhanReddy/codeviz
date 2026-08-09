TASK: T55 — MCP Debugging Tools (trace_call_path, get_callers_recursive, get_blast_radius)

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Add three new MCP tools that enable deep, AI-assisted debugging via graph
traversal. All three are pure graph algorithms on the existing CodeGraph.

Files to Modify:
- `codeviz-core/src/graph.rs`  (add all_paths(), callers_recursive(), blast_radius())
- `codeviz-mcp/src/tools.rs`  (add 3 new tool handlers + JSON schemas)
- `codeviz-mcp/src/server.rs` (register new tools in tools/list response)

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/mcp_debug_tools.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- No new Rust crates. Use only std::collections.
- all_paths() MUST have a hard cap on max_paths (default 10, max 50).
  Do NOT allow unbounded DFS on dense graphs.
- callers_recursive() and blast_radius() MUST detect cycles
  (use a `visited: HashSet<String>`) to avoid infinite loops.
- No unwrap() anywhere. Return Result<_, ParseError>.
- Write unit tests for each new graph method:
  - Linear chain: trace_call_path returns exactly 1 path.
  - Isolated node: blast_radius returns count 0.
  - Cyclic graph: no infinite loop.
- Ensure `cargo test --all` and `cargo clippy --all -- -D warnings` pass.

═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- For all_paths(): use recursive DFS with a `current_path: Vec<String>` and
  a `visited: HashSet<String>` to track the current path (prevents revisiting
  the same node in one path). Collect into `Vec<Vec<String>>`.
- For callers_recursive(): filter edges by `EdgeKind::Calls` where
  `edge.to_id == fn_name`. Recurse on each caller up to `depth` levels.
  Use a `global_visited: HashSet<String>` to handle cycles.
- For blast_radius(): BFS forward from the given node following
  `EdgeKind::Calls` where `edge.from_id == fn_name`.
