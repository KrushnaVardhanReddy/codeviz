TASK: T25 — Kotlin Parser

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement `LanguageParser` for `KotlinParser` per `docs/specs/parsers/kotlin.md`.
Use `tree-sitter-kotlin`.
Handle `suspend fun` as `is_async: true`.

Files to Modify/Create:


Spec (READ ONLY — implement from it, never edit):
  docs/specs/parsers/kotlin.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- CONTEXT: Use `tree-sitter-kotlin`. Be mindful of Kotlin's primary constructors and companion objects.
- Write comprehensive unit tests:
Parse the snippet in the Acceptance Criteria section of `docs/specs/parsers/kotlin.md`
and assert the exact node/edge counts specified.
- No unwraps or panics in core parsing logic. Return Result.
- Ensure 'cargo clippy --all -- -D warnings' and 'cargo test --all' pass cleanly.

═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- Use `tree-sitter-kotlin` crate.
- Kotlin has primary constructors inline with the class definition. Make sure you don't accidentally parse the constructor arguments as separate classes.
- Map Kotlin `object` declarations (singletons) and `companion object` to `NodeKind::Class` with a specific label or just `Class`.
