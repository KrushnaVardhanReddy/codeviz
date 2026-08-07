TASK: T22 — Watch Mode (`codeviz watch`)

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement `codeviz watch` per the spec:
- Use `notify` crate for cross-platform file watching
- 300ms debounce
- Print timestamped status on each update
- Continue watching after parse errors (never exit on error)
- Clean exit on Ctrl+C

Files to Modify/Create:


Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/watch_mode.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- CONTEXT: Use the `notify` crate for cross-platform file watching. Crucially, you MUST debounce the filesystem events to prevent the parser from thrashing on save.
- Write comprehensive unit tests:
- Test debounce logic: rapid file events produce single callback
- Test error in parse does not stop the watcher (mock error injection)
- No unwraps or panics in core parsing logic. Return Result.
- Ensure 'cargo clippy --all -- -D warnings' and 'cargo test --all' pass cleanly.
