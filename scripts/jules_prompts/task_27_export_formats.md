TASK: T27 — JSON / DOT Export Formats

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement `codeviz export` per the spec:
- `--format json`: serialize full `CodeGraph` as JSON (reuse serde)
- `--format dot`: generate Graphviz DOT with correct node shapes per `NodeKind`
- `--output -` writes to stdout
- No node truncation in export (unlike Mermaid renderer)

Files to Modify/Create:


Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/export_formats.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- CONTEXT: The JSON export must serialize the exact `CodeGraph` structure. DOT export should map nodes to `digraph` syntax cleanly.
- Write comprehensive unit tests:
- JSON export round-trips through `serde_json::from_str` without error
- DOT export contains `digraph codeviz {` header
- DOT output parses without error when piped to `dot -Tsvg` (integration test)
- `--output -` writes to stdout, not a file
- No unwraps or panics in core parsing logic. Return Result.
- Ensure 'cargo clippy --all -- -D warnings' and 'cargo test --all' pass cleanly.

═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- (Note: Task 26 might overlap with this. Ensure you are extending the export formats).
- For JSON, ensure the export matches the exact `CodeGraph` schema for external tools.
- For DOT format, ensure node IDs are wrapped in quotes if they contain special characters to avoid breaking Graphviz parsers.
