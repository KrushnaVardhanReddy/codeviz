# Jules Task 26 — Multiple Output Targets

## Spec
Read `docs/specs/features/multiple_outputs.md` before writing any code.
Read `docs/specs/08_config_schema.md` for the `[[outputs]]` config syntax.

## Files to Modify
- `codeviz-core/src/config.rs` (extend to support `[[outputs]]` array)
- `codeviz-cli/src/main.rs` (update `run` to iterate outputs)

## Requirements
Implement per the spec:
- Single parse, multiple injections
- Each output target can have its own `diagram_type`
- Per-target errors are logged but don't abort other targets
- CLI `--output` overrides the config `[[outputs]]` list entirely

## Unit Tests
- Config with 3 `[[outputs]]` entries parses into a `Vec<OutputTarget>`
- `run` with 2 targets updates both files
- One target with missing sentinel tags → error logged, second target still updated
