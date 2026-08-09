# Spec: Unused Module Detection (Phase 18)

## Overview
Detecting "dead code" (unused files/modules) is critical for codebase maintenance.
A module is considered unused if:
1. It is not an entry point (e.g., `main.rs`, `index.js`, or a script).
2. It has no incoming `Imports` edges from any other node in the graph.

## Graph Algorithm
Iterate through all `NodeKind::File` (and potentially other module/class nodes depending on the language) in the `CodeGraph`.
Check if there are any incoming `EdgeKind::Imports` where `to_id == node.id`.
If none exist, and the node is not explicitly marked as an entry point, it is unused.

## API Changes
Add a new method to `CodeGraph`:
```rust
impl CodeGraph {
    /// Returns a list of Node IDs that have no incoming dependencies.
    pub fn find_unused_modules(&self) -> Vec<String> {
        // ...
    }
}
```

## CLI Output
When running `codeviz run --find-unused`:
- Print a list of unused files to stdout.

## MCP Server Integration
Add a new MCP tool `find_unused_modules`:
- **Input:** `{ "path": "/path/to/repo" }`
- **Output:** JSON list of unused module nodes.
