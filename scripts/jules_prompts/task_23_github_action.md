# Jules Task 23 — GitHub Actions Marketplace Action

## Spec
Read `docs/specs/features/github_action.md` before writing any code.

## Files to Create
- `action.yml` (at repo root)
- `.github/workflows/npm-publish.yml` (npm publish on tag)
- `docs/github_action_usage.md` (end-user guide)

## Requirements
Implement the Marketplace action per the spec:
- `action.yml` with `path`, `output`, `diagram`, `commit` inputs
- Downloads the correct binary from GitHub Releases
- Commits updated diagram back if `commit: true`

## Tests
Add a test workflow `.github/workflows/action_test.yml` that uses the action
on the CodeViz repo itself to verify it works end-to-end.
