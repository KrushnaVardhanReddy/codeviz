# Jules Task 18 — Check Mode (`codeviz check`)

## Spec
Read `docs/specs/features/check_mode.md` before writing any code.
Read `docs/specs/05_cli_interface.md` for the CLI interface.

## Files to Modify
- `codeviz-cli/src/main.rs` (add `check` subcommand)
- `codeviz-core/src/check.rs` (new: comparison logic)

## Requirements
Implement `codeviz check` per the spec. This is read-only — must never write to disk.
Whitespace normalize before comparing as specified.

## Unit Tests
- `check` on matching diagrams returns `Ok(true)`
- `check` on stale diagrams returns `Ok(false)` with a diff
- Assert the output file is never written to
