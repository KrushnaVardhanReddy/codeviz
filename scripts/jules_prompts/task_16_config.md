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
- CONTEXT: Use `serde` with `toml` crate. If `codeviz.toml` is missing, gracefully fallback to default configuration values without erroring.
- Write comprehensive unit tests:
- Parse a complete `codeviz.toml` and assert all fields match expected values
- Parse a partial config and assert missing fields use defaults
- Simulate CLI flag override and assert it takes precedence over config value
- No unwraps or panics in core parsing logic. Return Result.
- Ensure 'cargo clippy --all -- -D warnings' and 'cargo test --all' pass cleanly.

═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- Use `serde` and the `toml` crate to parse `codeviz.toml`.
- Define a `Config` struct with `Default` implemented. Use `#[serde(default)]` extensively so that missing fields automatically fall back to their defaults.
- Place the config parsing logic in `codeviz-core` so that both the CLI and WASM adapters can use it.
