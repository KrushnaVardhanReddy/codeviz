# Jules Task 13 — Go Parser

## Spec
Read `docs/specs/parsers/go.md` before writing any code.

## Files to Create
- `codeviz-go/Cargo.toml`
- `codeviz-go/src/lib.rs`
- `codeviz-go/src/parser.rs`
- Update workspace `Cargo.toml` to include this crate

## Requirements
Implement `LanguageParser` for `GoParser` per `docs/specs/parsers/go.md`.
Use `tree-sitter-go`.

## Unit Tests
Parse the snippet in the Acceptance Criteria section of `docs/specs/parsers/go.md`
and assert the exact node/edge counts specified.
