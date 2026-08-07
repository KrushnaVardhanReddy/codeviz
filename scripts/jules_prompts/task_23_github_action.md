TASK: T23 — GitHub Actions Marketplace Action

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement the Marketplace action per the spec:
- `action.yml` with `path`, `output`, `diagram`, `commit` inputs
- Downloads the correct binary from GitHub Releases
- Commits updated diagram back if `commit: true`

Files to Modify/Create:


Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/github_action.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- Write comprehensive unit tests:

- No unwraps or panics in core parsing logic. Return Result.
- Ensure 'cargo clippy --all -- -D warnings' and 'cargo test --all' pass cleanly.
