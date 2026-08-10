TASK: T45 — Code Coverage Overlay

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Allow users to overlay standard LCOV code coverage data onto the `CodeGraph`,
visually highlighting which critical modules lack test coverage. Add a
`--coverage-file` flag to the CLI and augment the MCP `get_module_graph` tool.

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/code_coverage.md

═══════════════════════════════════════════════════════════════
BACKGROUND — WHAT ALREADY EXISTS
═══════════════════════════════════════════════════════════════

- `Node` struct is in `codeviz-core/src/graph.rs`. It may already have a `meta` field.
- Mermaid rendering is in `codeviz-core/src/render/mermaid.rs`.
- MCP `get_module_graph` tool is registered in `codeviz-mcp/src/tools.rs`.
- The CLI `run` subcommand is in `codeviz-cli/src/main.rs`.

═══════════════════════════════════════════════════════════════
DELIVERABLES
═══════════════════════════════════════════════════════════════

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. CREATE: codeviz-core/src/coverage.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Implement an LCOV parser that reads an `lcov.info` file and returns a
`HashMap<String, f64>` mapping file paths to their line coverage percentage (0.0–100.0).

LCOV format example:
```
SF:src/auth.rs
DA:10,1
DA:11,0
DA:12,1
end_of_record
```
Parse `SF:` as the file path and `DA:<line>,<hit_count>` to calculate `% lines hit`.

Register in `codeviz-core/src/lib.rs`.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
2. MODIFY: codeviz-core/src/graph.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Add an optional `coverage_percent: Option<f64>` field to `Node` or `NodeMeta`.
Add a method `apply_coverage(&mut self, coverage: &HashMap<String, f64>)` that
walks through all `NodeKind::File` nodes and sets `coverage_percent` by matching
the node's `file_path` against the coverage map.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
3. MODIFY: codeviz-core/src/render/mermaid.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
When `coverage_percent` is set on a node, use Mermaid `style` directives to color it:
- Green (`#22c55e`): coverage ≥ 80%
- Yellow (`#eab308`): coverage 50–80%
- Red (`#ef4444`): coverage < 50%
- Gray (no style): `coverage_percent` is `None`.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
4. MODIFY: CLI & MCP
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
CLI: Add `--coverage-file <path>` to `run` subcommand. When provided, parse the
LCOV file and apply coverage to the graph before rendering.

MCP: Update `get_module_graph` to accept an optional `coverage_file` string parameter.
If provided, apply coverage to the returned graph JSON.

═══════════════════════════════════════════════════════════════
CRITICAL IMPLEMENTATION RULES
═══════════════════════════════════════════════════════════════
1. Gracefully handle files in the LCOV report that are not in the graph (just skip them).
2. A missing or invalid LCOV file must not crash the CLI — print a warning and continue.
3. Write a unit test for the LCOV parser with the example above.
4. Ensure `cargo test --all` and `cargo clippy --all -- -D warnings` pass.

Commit: "jules: T45 — LCOV code coverage overlay on CodeGraph"
Target branch: feat-t45-code-coverage
