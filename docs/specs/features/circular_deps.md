# Spec: Circular Dependency Detection (Phase 18)

## Overview
Circular dependencies (A -> B -> C -> A) are a major source of technical debt, tight coupling, and runtime errors in many languages (e.g., Python, Node.js). 
CodeViz needs the ability to detect and highlight import cycles within the codebase graph.

## Graph Algorithm
We already extract all `Imports` edges in the `CodeGraph`. 
A cycle detection algorithm should run as a post-processing step on the `CodeGraph` after parsing is complete.

1.  **Tarjan's strongly connected components algorithm** OR a simple Depth-First Search (DFS) with a visited stack.
2.  Filter the edges to only consider `EdgeKind::Imports`.
3.  Identify cycles and extract the exact path (e.g., `a.py -> b.py -> c.py -> a.py`).

## API Changes
Add a new method to `CodeGraph`:
```rust
impl CodeGraph {
    /// Returns a list of cycles, where each cycle is a vector of Node IDs.
    pub fn find_import_cycles(&self) -> Vec<Vec<String>> {
        // ...
    }
}
```

## CLI Output
When running `codeviz run --detect-cycles`:
- If cycles are found, print a warning to stderr listing the cycles and return a non-zero exit code.
- If rendering a Markdown diagram, color the cycle edges in Red (e.g., using Mermaid's `linkStyle`).

## MCP Server Integration
Add a new MCP tool `find_circular_dependencies`:
- **Input:** `{ "path": "/path/to/repo" }`
- **Output:** JSON list of cycles.
