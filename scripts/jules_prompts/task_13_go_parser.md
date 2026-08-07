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

═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- Use `tree-sitter-go` crate.
- Go's `import` statements are often grouped in `import ( ... )` blocks. Make sure your AST traversal iterates over all children of the import declaration.
- Structs and Interfaces in Go are defined using `type X struct/interface`. Ensure you map these to `NodeKind::Class` and `NodeKind::Interface` respectively.
