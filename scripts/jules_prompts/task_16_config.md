TASK: T16 — codeviz.toml Config

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement config loading per `docs/specs/08_config_schema.md`.
Use `serde` + `toml` crate. Apply CLI flag precedence as specified.

Files to Modify/Create:


Spec (READ ONLY — implement from it, never edit):
  docs/specs/08_config_schema.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- Write comprehensive unit tests:
- Parse a complete `codeviz.toml` and assert all fields match expected values
- Parse a partial config and assert missing fields use defaults
- Simulate CLI flag override and assert it takes precedence over config value
- No unwraps or panics in core parsing logic. Return Result.
- Ensure 'cargo clippy --all -- -D warnings' and 'cargo test --all' pass cleanly.
