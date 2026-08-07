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
- CONTEXT: The `action.yml` should define inputs for the path to scan. The action will execute the pre-compiled codeviz binary.
- Write comprehensive unit tests:

- No unwraps or panics in core parsing logic. Return Result.
- Ensure 'cargo clippy --all -- -D warnings' and 'cargo test --all' pass cleanly.

═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- Define an `action.yml` file in the root of the repository.
- Use a `runs: using: 'composite'` action that downloads the latest pre-compiled CodeViz binary from GitHub Releases and executes `codeviz run` on the user's workspace.
- Allow inputs like `config-path` and `fail-on-violation`.
