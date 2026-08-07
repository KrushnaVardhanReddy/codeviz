# Jules Task 16 — codeviz.toml Config

## Spec
Read `docs/specs/08_config_schema.md` before writing any code.

## Files to Create/Modify
- `codeviz-core/src/config.rs`
- `codeviz-cli/src/main.rs` (load config, merge with CLI flags)
- `codeviz.toml.example` (at repo root)

## Requirements
Implement config loading per `docs/specs/08_config_schema.md`.
Use `serde` + `toml` crate. Apply CLI flag precedence as specified.

## Unit Tests
- Parse a complete `codeviz.toml` and assert all fields match expected values
- Parse a partial config and assert missing fields use defaults
- Simulate CLI flag override and assert it takes precedence over config value
