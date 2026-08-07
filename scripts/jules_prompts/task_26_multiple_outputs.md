TASK: T26 — Multiple Output Targets

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement per the spec:
- Single parse, multiple injections
- Each output target can have its own `diagram_type`
- Per-target errors are logged but don't abort other targets
- CLI `--output` overrides the config `[[outputs]]` list entirely

Files to Modify/Create:


Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/multiple_outputs.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- Write comprehensive unit tests:
- Config with 3 `[[outputs]]` entries parses into a `Vec<OutputTarget>`
- `run` with 2 targets updates both files
- One target with missing sentinel tags → error logged, second target still updated
- No unwraps or panics in core parsing logic. Return Result.
- Ensure 'cargo clippy --all -- -D warnings' and 'cargo test --all' pass cleanly.
