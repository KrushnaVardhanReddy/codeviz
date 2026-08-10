TASK: T37 — Blast Radius & Impact Analysis

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement a Blast Radius feature that calculates the transitive closure of all
modules and functions that depend on a modified node. This is critical for PR reviews.

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/blast_radius.md

═══════════════════════════════════════════════════════════════
BACKGROUND — WHAT ALREADY EXISTS
═══════════════════════════════════════════════════════════════

- `CodeGraph` exists in `codeviz-core/src/graph.rs`.
- Output rendering logic exists in `codeviz-core/src/render/`.

═══════════════════════════════════════════════════════════════
DELIVERABLES
═══════════════════════════════════════════════════════════════

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. MODIFY: codeviz-core/src/graph.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Add a `blast_radius_subgraph(&self, start_file_path: &str) -> CodeGraph` method.
- Perform a backward breadth-first search (BFS) on the graph starting from all nodes in the given file.
- Extract a subgraph containing only the path from the modified nodes up to their root callers.
- Add metadata or tags to the nodes indicating if they are the "source" (modified) or "affected".

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
2. MODIFY: codeviz-core/src/render/mermaid.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Ensure the Mermaid renderer highlights the source nodes in RED and affected dependents in ORANGE.
Use Mermaid `classDef` and `class` syntax.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
3. MODIFY: codeviz-cli/src/main.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Add a `--impact <file_path>` flag to the `run` and `diff` subcommands.
When provided, instead of outputting the full graph, output the `blast_radius_subgraph`.

═══════════════════════════════════════════════════════════════
CRITICAL IMPLEMENTATION RULES
═══════════════════════════════════════════════════════════════
1. The BFS must detect cycles and avoid infinite loops.
2. Output must correctly serialize as JSON and Mermaid.
3. Ensure `cargo test --all` and `cargo clippy --all -- -D warnings` pass.

Commit: "jules: T37 — Blast Radius and Impact Analysis subgraph extraction"
Target branch: feat-t37-blast-radius
