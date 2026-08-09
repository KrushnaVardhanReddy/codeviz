# Spec: MCP Debugging Tools — T55 (MVP v1)

## Overview
Three new MCP tools that supercharge AI-assisted debugging by exposing deep
graph traversal capabilities. All tools are pure graph algorithms on the
existing `CodeGraph` — no new data sources required.

## New Tools

### 1. `trace_call_path`
Returns ALL paths (not just shortest) from an entry point to a target function.
Answers: *"How does execution reach this function?"*

```json
Input:  { "from": "main", "to": "db_connect", "path": string, "max_paths": number? }
Output: {
  "paths": [
    ["main", "handle_request", "auth_check", "db_connect"],
    ["main", "health_check", "db_connect"]
  ],
  "count": 2
}
```
- `max_paths` default: 10. Cap at 50 to prevent combinatorial explosion.
- Uses DFS with backtracking on the `Calls` edge subgraph.

### 2. `get_callers_recursive`
Returns the full N-level-deep caller chain for a function.
Answers: *"Who originally triggered this call, N levels up?"*

```json
Input:  { "fn_name": "parse_token", "path": string, "depth": number? }
Output: {
  "call_tree": {
    "node": "parse_token",
    "callers": [
      {
        "node": "auth_check",
        "callers": [{ "node": "handle_request", "callers": [] }]
      }
    ]
  }
}
```
- `depth` default: 3. Max: 10.
- Returns a recursive tree structure, not a flat list.

### 3. `get_blast_radius`
Returns all functions transitively reachable FROM a given node (forward reachability).
Answers: *"If I change this function, what else could break?"*

```json
Input:  { "fn_name": "parse_token", "path": string }
Output: {
  "affected_nodes": ["auth_check", "handle_request", "middleware"],
  "count": 3,
  "max_depth_reached": 4
}
```
- Performs forward BFS from the given node following `Calls` edges.
- Returns all transitively reachable nodes, not just direct callees.

## Files to Modify
- `codeviz-core/src/graph.rs` — add `all_paths()`, `callers_recursive()`, `blast_radius()` methods to `CodeGraph`
- `codeviz-mcp/src/tools.rs` — add 3 new tool handlers + JSON schemas
- `codeviz-mcp/src/server.rs` — register new tools in `tools/list`

## Constraints
- No new Rust crates. Implement using standard `std::collections`.
- `all_paths()` must have a hard cap (`max_paths`) to avoid combinatorial explosion on dense graphs.
- `callers_recursive()` must detect and short-circuit cycles to avoid infinite loops.
- `get_blast_radius()` must detect and short-circuit cycles.
- No `unwrap()` — return `Result<..., ParseError>`.
- Ensure `cargo test --all` and `cargo clippy --all -- -D warnings` pass.
- Write unit tests for each of the three new graph methods.

## Acceptance Criteria
- `trace_call_path("main", "leaf_fn")` on a linear call chain returns exactly 1 path.
- `get_callers_recursive` with `depth: 2` returns at most 2 levels of callers.
- `get_blast_radius` on an isolated node returns `count: 0`.
- Cyclic graphs do not cause infinite loops in any of the three methods.
- `tools/list` response includes all 3 new tool definitions with correct schemas.
