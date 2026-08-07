TASK: T18 — Check Mode (`codeviz check`)

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement `codeviz check` per the spec. This is read-only — must never write to disk.
Whitespace normalize before comparing as specified.

Files to Modify/Create:


Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/check_mode.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- CONTEXT: `codeviz check` should exit with code 1 if there are architectural violations. This is designed for CI pipelines.
- Write comprehensive unit tests:
- `check` on matching diagrams returns `Ok(true)`
- `check` on stale diagrams returns `Ok(false)` with a diff
- Assert the output file is never written to
- No unwraps or panics in core parsing logic. Return Result.
- Ensure 'cargo clippy --all -- -D warnings' and 'cargo test --all' pass cleanly.
