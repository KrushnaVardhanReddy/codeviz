# Spec: Code Health Score (Phase 18)

## Overview
A composite metric (1 to 10, where 10 is perfectly healthy) assigned to each module/file to provide an intuitive assessment of maintainability.
This helps teams identify technical debt and monitor codebase degradation over time.

## Metric Calculation
The score will be a simple heuristic based on the `CodeGraph` metrics we already have. 
Each file starts with a score of 10.0, and points are deducted for:

1.  **High Outbound Coupling (Fan-out):** File imports many other files (deduct for `> 10` outgoing `Imports` edges).
2.  **High Inbound Coupling (Fan-in):** File is imported by many other files (deduct for `> 15` incoming `Imports` edges).
3.  **Large File Size (Proxy):** Many nodes defined within the file (deduct for `> 20` `Node` instances in the same file).
4.  **Cyclic Dependencies:** If the file is part of a circular dependency (deduct a severe penalty, e.g., -3.0).

Score must be clamped between 1.0 and 10.0.

## API Changes
Add a new method to `CodeGraph`:
```rust
impl CodeGraph {
    /// Returns a map of Node ID to its health score (1.0 - 10.0).
    pub fn compute_health_scores(&self) -> std::collections::HashMap<String, f64> {
        // ...
    }
}
```

## CLI Output
When running `codeviz run --health`:
- Print a tabulated report of all files and their health scores to stdout, sorting the unhealthiest files first.

## MCP Server Integration
Add a new MCP tool `get_code_health`:
- **Input:** `{ "path": "/path/to/repo" }`
- **Output:** JSON mapping of files to their health scores.
