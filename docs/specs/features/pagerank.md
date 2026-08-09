# Spec: PageRank & Centrality Scoring (Phase 18)

## Overview
To determine the most "influential" or critical modules in a codebase, we can apply the PageRank algorithm to the `CodeGraph`. 
A module with a high PageRank score is a core dependency; if it breaks or is modified, the blast radius is large.

## Graph Algorithm
1. Extract all nodes and `EdgeKind::Imports` edges.
2. Reverse the edge direction for the PageRank calculation (so that a module being imported by many other modules receives a high score).
3. Implement a standard iterative PageRank algorithm:
   - Initialize all node scores to `1.0 / N` (where N is total nodes).
   - Iterate a fixed number of times (e.g., 20 iterations) or until convergence.
   - `NewScore(A) = (1 - d)/N + d * sum(Score(B) / OutDegree(B))` for all B that import A, where `d` is the damping factor (typically 0.85).

## API Changes
Add a new method to `CodeGraph`:
```rust
impl CodeGraph {
    /// Returns a map of Node ID to its PageRank score.
    pub fn compute_pagerank(&self) -> std::collections::HashMap<String, f64> {
        // ...
    }
}
```

## CLI Output
When running `codeviz run --pagerank`:
- Print the top 10 most critical files to stdout, sorted by PageRank score.

## MCP Server Integration
Add a new MCP tool `get_critical_modules`:
- **Input:** `{ "path": "/path/to/repo", "limit": 10 }`
- **Output:** JSON list of the most critical module nodes with their scores.
