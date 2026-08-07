# Jules Task 15 — Java Parser

## Spec
Read `docs/specs/parsers/java.md` before writing any code.

## Files to Create
- `codeviz-java/Cargo.toml`
- `codeviz-java/src/lib.rs`
- `codeviz-java/src/parser.rs`
- Update workspace `Cargo.toml` to include this crate

## Requirements
Implement `LanguageParser` for `JavaParser` per `docs/specs/parsers/java.md`.
Use `tree-sitter-java`.

## Unit Tests
Parse the snippet in the Acceptance Criteria section of `docs/specs/parsers/java.md`
and assert the exact node/edge counts specified.
