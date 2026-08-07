# Jules Task 21 — Install Hook (`codeviz install-hook`)

## Spec
Read `docs/specs/features/install_hook.md` before writing any code.
Read `docs/specs/05_cli_interface.md` for the CLI interface.

## Files to Modify
- `codeviz-cli/src/main.rs` (add `install-hook` subcommand)

## Requirements
Implement `codeviz install-hook` per the spec.
Idempotent — running twice must not produce duplicate entries.

## Unit Tests
- Test with no existing `.pre-commit-config.yaml` → file is created
- Test with existing file without codeviz entry → entry is appended
- Test with existing file with codeviz entry → skipped (idempotent)
- Test that sentinel tags are only added if absent
