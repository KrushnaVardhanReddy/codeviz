TASK: T20 — Graph Diff Mode (`codeviz diff`)

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement `codeviz diff` per the spec:
- Use `git archive` for base ref (no working tree pollution)
- Compute node/edge deltas
- Support `--format human|mermaid|json`

Files to Modify/Create:


Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/graph_diff.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- Write comprehensive unit tests:
- Diff two identical `CodeGraph`s → all delta fields empty
- Diff graph with one extra node → `added_nodes` has 1 entry
- Diff graph with removed edge → `removed_edges` has 1 entry
- No unwraps or panics in core parsing logic. Return Result.
- Ensure 'cargo clippy --all -- -D warnings' and 'cargo test --all' pass cleanly.
