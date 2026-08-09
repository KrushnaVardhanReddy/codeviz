# Jules Task 01 — Rust Workspace Init

## Objective
Initialize the CodeViz Rust workspace with four crates.

## Files to Create
- `Cargo.toml` (workspace root)
- `codeviz-core/Cargo.toml` + `codeviz-core/src/lib.rs`
- `codeviz-cli/Cargo.toml` + `codeviz-cli/src/main.rs`
- `codeviz-wasm/Cargo.toml` + `codeviz-wasm/src/lib.rs`
- `codeviz-mcp/Cargo.toml` + `codeviz-mcp/src/lib.rs`

## Requirements
1. Workspace root `Cargo.toml` must declare all four members.
2. `codeviz-core` is a pure library crate (no OS I/O, no file system access).
3. `codeviz-cli` depends on `codeviz-core`. Entry point: `codeviz --help`.
4. `codeviz-wasm` has `crate-type = ["cdylib"]` for wasm-pack.
5. `codeviz-mcp` depends on `codeviz-core`. Stub the MCP server entry point.
6. Add `.gitignore` entries for `target/` and `*.wasm`.
7. `cargo build` must succeed with zero errors and zero warnings.
8. Add a basic dummy unit test in each crate's `lib.rs` or `main.rs` to verify test runners work.


═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- Use stable Rust 2021 edition. Do NOT use unstable features like `let_chains`.
- Always run `cargo test --all` and `cargo clippy --all -- -D warnings` before completing the task.
