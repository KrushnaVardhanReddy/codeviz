TASK: T15 — Java Parser

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement `LanguageParser` for `JavaParser` per `docs/specs/parsers/java.md`.
Use `tree-sitter-java`.

Files to Modify/Create:


Spec (READ ONLY — implement from it, never edit):
  docs/specs/parsers/java.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- CONTEXT: Use `tree-sitter-java`. Capture class inheritance (`extends`) and interfaces (`implements`) properly.
- Write comprehensive unit tests:
Parse the snippet in the Acceptance Criteria section of `docs/specs/parsers/java.md`
and assert the exact node/edge counts specified.
- No unwraps or panics in core parsing logic. Return Result.
- Ensure 'cargo clippy --all -- -D warnings' and 'cargo test --all' pass cleanly.

═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- Use `tree-sitter-java` crate.
- Java `package` declarations should ideally scope the node names, but for V1, just use the file path + class name as the unique ID.
- Properly capture `extends` as `EdgeKind::Inherits` and `implements` as `EdgeKind::Implements`.
