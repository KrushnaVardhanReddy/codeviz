TASK: T37 — Blast Radius & Impact Analysis

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Calculate the "Blast Radius" of changes by performing a backward traversal on the `CodeGraph`.

Files to Modify/Create:
- `codeviz-core/src/impact.rs`
- `codeviz-cli/src/main.rs` (add --impact flag)

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/blast_radius.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- Implement a Breadth-First Search (BFS) starting from the given node(s) and traversing edges backwards (e.g., finding all callers of a function).
- Output the resulting subgraph.

═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- The BFS should follow `Calls`, `Imports`, and `Inherits` edges in reverse to find what depends on the target node.
