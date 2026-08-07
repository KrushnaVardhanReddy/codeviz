TASK: T14 — Rust Parser

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement `LanguageParser` for `RustLangParser` per `docs/specs/parsers/rust_lang.md`.
Use `tree-sitter-rust`.
Handle workspace multi-crate support as specified.

Files to Modify/Create:


Spec (READ ONLY — implement from it, never edit):
  docs/specs/parsers/rust_lang.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- CONTEXT: Use `tree-sitter-rust`. Pay special attention to `mod` vs `use` for module resolution.
- Write comprehensive unit tests:
Parse the snippet in the Acceptance Criteria section of `docs/specs/parsers/rust_lang.md`
and assert the exact node/edge counts specified.
- No unwraps or panics in core parsing logic. Return Result.
- Ensure 'cargo clippy --all -- -D warnings' and 'cargo test --all' pass cleanly.

═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- Use `tree-sitter-rust` crate.
- Rust module resolution is tricky. For `mod my_module;`, create an `Imports` edge. For `use std::collections::HashMap;`, also create an `Imports` edge but try to extract the base module.
- Treat Rust `trait` as `NodeKind::Interface` and `struct`/`enum` as `NodeKind::Class`.
