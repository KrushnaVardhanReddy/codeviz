# Spec: Blast Radius & Impact Analysis (Phase 16)

## Overview
When a developer modifies a module, CodeViz should calculate the "Blast Radius" — the 
transitive closure of all modules and functions that depend on the modified node. This 
is critical for PR reviews (like CodeSee's Review Maps).

## Requirements
- Add a `--impact <file_path>` flag to `codeviz run` and `codeviz diff`.
- Engine performs a backward breadth-first search (BFS) on the `CodeGraph` starting from the modified nodes.
- Extracts a subgraph containing only the path from the modified nodes up to their root callers.
- The output (JSON or Mermaid) highlights the modified nodes in RED and the affected dependents in ORANGE.
