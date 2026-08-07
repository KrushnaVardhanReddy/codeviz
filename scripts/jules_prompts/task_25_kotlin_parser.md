# Jules Task 25 — Kotlin Parser

## Spec
Read `docs/specs/parsers/kotlin.md` before writing any code.

## Files to Create
- `codeviz-kotlin/Cargo.toml`
- `codeviz-kotlin/src/lib.rs`
- `codeviz-kotlin/src/parser.rs`
- Update workspace `Cargo.toml` to include this crate

## Requirements
Implement `LanguageParser` for `KotlinParser` per `docs/specs/parsers/kotlin.md`.
Use `tree-sitter-kotlin`.
Handle `suspend fun` as `is_async: true`.

## Unit Tests
Parse the snippet in the Acceptance Criteria section of `docs/specs/parsers/kotlin.md`
and assert the exact node/edge counts specified.
