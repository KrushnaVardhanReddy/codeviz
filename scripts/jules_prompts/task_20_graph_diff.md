# Jules Task 20 — Graph Diff Mode (`codeviz diff`)

## Spec
Read `docs/specs/features/graph_diff.md` before writing any code.
Read `docs/specs/05_cli_interface.md` for the CLI interface.

## Files to Create/Modify
- `codeviz-core/src/diff.rs` (new: graph delta computation)
- `codeviz-cli/src/main.rs` (add `diff` subcommand)

## Requirements
Implement `codeviz diff` per the spec:
- Use `git archive` for base ref (no working tree pollution)
- Compute node/edge deltas
- Support `--format human|mermaid|json`

## Unit Tests
- Diff two identical `CodeGraph`s → all delta fields empty
- Diff graph with one extra node → `added_nodes` has 1 entry
- Diff graph with removed edge → `removed_edges` has 1 entry
