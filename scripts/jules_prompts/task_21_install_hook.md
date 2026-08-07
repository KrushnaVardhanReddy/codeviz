TASK: T21 — Install Hook (`codeviz install-hook`)

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement `codeviz install-hook` per the spec.
Idempotent — running twice must not produce duplicate entries.

Files to Modify/Create:


Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/install_hook.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- Write comprehensive unit tests:
- Test with no existing `.pre-commit-config.yaml` → file is created
- Test with existing file without codeviz entry → entry is appended
- Test with existing file with codeviz entry → skipped (idempotent)
- Test that sentinel tags are only added if absent
- No unwraps or panics in core parsing logic. Return Result.
- Ensure 'cargo clippy --all -- -D warnings' and 'cargo test --all' pass cleanly.
