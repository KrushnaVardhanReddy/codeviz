# Jules Task 07 — CLI: codeviz run

## Objective
Wire the Python parser into the CLI adapter so `codeviz run` works end-to-end.

## Files to Modify
- `codeviz-cli/src/main.rs`
- `codeviz-cli/Cargo.toml` (add `codeviz-core`, `codeviz-python` deps)

## CLI Interface to Implement
```
codeviz run --path <dir> --output <file.md> [--diagram module|call|class] [--depth N]
```
- `--path`: directory to scan recursively for source files
- `--output`: markdown file to inject the diagram into (must have sentinel tags)
- `--diagram`: diagram type (default: `module`)
- `--depth`: maximum graph depth (default: unlimited)

## Requirements
1. Walk `--path` recursively, collect files by extension.
2. Dispatch each file to the `LanguageRegistry`.
3. Merge all per-file `CodeGraph`s into one.
4. Render via `MermaidRenderer` with the selected `DiagramKind`.
5. Inject into `--output` using `inject_mermaid`.
6. Print summary: files parsed, nodes, edges, output path.
7. Exit code 0 on success, 1 on any error.

## Unit Tests
- Test CLI argument parsing for all flags
- Test that `--depth` properly truncates the graph at the given level
- Test that missing `--output` file prints a clear error and exits with code 1


═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- Use stable Rust 2021 edition. Do NOT use unstable features like `let_chains`.
- Always run `cargo test --all` and `cargo clippy --all -- -D warnings` before completing the task.
