# Jules Task 22 — Watch Mode (`codeviz watch`)

## Spec
Read `docs/specs/features/watch_mode.md` before writing any code.
Read `docs/specs/05_cli_interface.md` for the CLI interface.

## Files to Create/Modify
- `codeviz-cli/src/main.rs` (add `watch` subcommand)
- `codeviz-cli/Cargo.toml` (add `notify` crate dependency)

## Requirements
Implement `codeviz watch` per the spec:
- Use `notify` crate for cross-platform file watching
- 300ms debounce
- Print timestamped status on each update
- Continue watching after parse errors (never exit on error)
- Clean exit on Ctrl+C

## Unit Tests
- Test debounce logic: rapid file events produce single callback
- Test error in parse does not stop the watcher (mock error injection)
