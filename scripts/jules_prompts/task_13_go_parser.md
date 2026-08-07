TASK: T13 — Go Parser

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement `LanguageParser` for `GoParser` per `docs/specs/parsers/go.md`.
Use `tree-sitter-go`.

Files to Modify/Create:


Spec (READ ONLY — implement from it, never edit):
  docs/specs/parsers/go.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- CONTEXT: Use `tree-sitter-go`. Ensure you correctly capture Go's package-level imports and structs. Do not run `cargo clippy --fix` if it injects unstable features.
- Write comprehensive unit tests:
Parse the snippet in the Acceptance Criteria section of `docs/specs/parsers/go.md`
and assert the exact node/edge counts specified.
- No unwraps or panics in core parsing logic. Return Result.
- Ensure 'cargo clippy --all -- -D warnings' and 'cargo test --all' pass cleanly.
