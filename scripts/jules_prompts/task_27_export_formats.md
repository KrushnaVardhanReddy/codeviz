# Jules Task 27 — JSON / DOT Export Formats

## Spec
Read `docs/specs/features/export_formats.md` before writing any code.
Read `docs/specs/05_cli_interface.md` for the `export` subcommand interface.

## Files to Create/Modify
- `codeviz-core/src/export/mod.rs` (new)
- `codeviz-core/src/export/dot.rs` (new: DOT format renderer)
- `codeviz-cli/src/main.rs` (add `export` subcommand)

## Requirements
Implement `codeviz export` per the spec:
- `--format json`: serialize full `CodeGraph` as JSON (reuse serde)
- `--format dot`: generate Graphviz DOT with correct node shapes per `NodeKind`
- `--output -` writes to stdout
- No node truncation in export (unlike Mermaid renderer)

## Unit Tests
- JSON export round-trips through `serde_json::from_str` without error
- DOT export contains `digraph codeviz {` header
- DOT output parses without error when piped to `dot -Tsvg` (integration test)
- `--output -` writes to stdout, not a file
