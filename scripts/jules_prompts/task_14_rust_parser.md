# Jules Task 14 — Rust Parser

## Spec
Read `docs/specs/parsers/rust_lang.md` before writing any code.

## Files to Create
- `codeviz-rust-lang/Cargo.toml`
- `codeviz-rust-lang/src/lib.rs`
- `codeviz-rust-lang/src/parser.rs`
- Update workspace `Cargo.toml` to include this crate

## Requirements
Implement `LanguageParser` for `RustLangParser` per `docs/specs/parsers/rust_lang.md`.
Use `tree-sitter-rust`.
Handle workspace multi-crate support as specified.

## Unit Tests
Parse the snippet in the Acceptance Criteria section of `docs/specs/parsers/rust_lang.md`
and assert the exact node/edge counts specified.
