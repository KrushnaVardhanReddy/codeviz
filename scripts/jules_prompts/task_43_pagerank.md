TASK: T43 — PageRank & Centrality Scoring

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement PageRank centrality scoring on the `CodeGraph` to identify the most
"influential" or critical modules. Expose via CLI (`--pagerank`) and a new MCP
tool (`get_critical_modules`).

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/pagerank.md

═══════════════════════════════════════════════════════════════
BACKGROUND — WHAT ALREADY EXISTS
═══════════════════════════════════════════════════════════════

- `CodeGraph` is defined in `codeviz-core/src/graph.rs`.
- `EdgeKind::Imports` is the edge type to use for this calculation.

═══════════════════════════════════════════════════════════════
DELIVERABLES
═══════════════════════════════════════════════════════════════

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. MODIFY: codeviz-core/src/graph.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Implement the following method using a standard iterative PageRank algorithm:

```rust
/// Returns a map of Node ID → PageRank score.
/// Uses damping factor d=0.85, 20 iterations (or until convergence delta < 1e-6).
pub fn compute_pagerank(&self) -> std::collections::HashMap<String, f64> {
    // 1. Build an adjacency map of REVERSED Imports edges.
    //    (A node's score increases if many other nodes import it.)
    // 2. Initialize all scores to 1.0 / N.
    // 3. Iterate: NewScore(A) = (1-d)/N + d * sum(Score(B) / OutDegree(B))
    //    for all B that have an edge pointing to A (i.e. B imports A).
    // 4. Repeat for 20 iterations or until max delta < 1e-6.
}
```

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
2. MODIFY: codeviz-cli/src/main.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Add a `--pagerank` flag to the `run` subcommand.
When enabled:
- Call `compute_pagerank()` after generating the graph.
- Print the top 10 most critical files to stdout, sorted by score descending.
- Format: `  1. src/auth.rs  (score: 0.04521)`

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
3. MODIFY: codeviz-mcp/src/tools.rs & server.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Register a new MCP tool:
- Name: `get_critical_modules`
- Input: `{ "path": string, "limit": number? }` (default limit: 10)
- Output: `{ "modules": [{ "node_id": string, "file_path": string, "score": float }] }`

═══════════════════════════════════════════════════════════════
CRITICAL IMPLEMENTATION RULES
═══════════════════════════════════════════════════════════════
1. Use only `std::collections::HashMap` — no external graph or linear algebra crates.
2. Handle dangling nodes (no outbound edges) with the standard teleportation approach.
3. Write a unit test: in a linear chain A→B→C, node B should have a higher PageRank than A.
4. Ensure `cargo test --all` and `cargo clippy --all -- -D warnings` pass.

Commit: "jules: T43 — PageRank centrality scoring engine and MCP tool"
Target branch: feat-t43-pagerank
