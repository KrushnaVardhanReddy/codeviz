TASK: T45 — Code Coverage Overlay

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Parse LCOV files to annotate the CodeGraph with code coverage percentages and color-code the Mermaid output.

Files to Modify/Create:
- `codeviz-core/src/ir.rs` (add `coverage_percent: Option<f64>` to `Node`)
- `codeviz-core/src/coverage.rs` (new module to parse LCOV and annotate `CodeGraph`)
- `codeviz-cli/src/main.rs` (add `--coverage-file` flag)
- `codeviz-core/src/render/mermaid.rs` (add styling for coverage colors)
- `codeviz-mcp/src/tools.rs` (update `get_module_graph`)

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/code_coverage.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- Implement a basic LCOV parser (look for `SF:` for source file, `DA:` for execution counts). Calculate line coverage percentage.
- Match LCOV `SF` paths to `Node.file_path`. Be resilient to absolute vs relative path differences if possible.
- Update the Mermaid renderer to use `style` lines: `<50%` = red, `50-80%` = yellow, `>80%` = green.
- Ensure `cargo clippy --all -- -D warnings` and `cargo test --all` pass.
- Write unit tests for LCOV parsing and graph annotation.


═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- Use stable Rust 2021 edition. Do NOT use unstable features like `let_chains`.
- Always run `cargo test --all` and `cargo clippy --all -- -D warnings` before completing the task.
