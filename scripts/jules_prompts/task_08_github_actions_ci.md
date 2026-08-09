# Jules Task 08 — GitHub Actions CI

## Objective
Set up GitHub Actions CI for the CodeViz project.

## Files to Create
- `.github/workflows/ci.yml`
- `.github/workflows/release.yml` (binary releases on tag push)

## ci.yml Requirements
Trigger: push to any branch, PR to main.
Jobs:
1. `test` — `cargo test --all` on ubuntu-latest, macos-latest, windows-latest
2. `lint` — `cargo clippy --all -- -D warnings` + `cargo fmt --check`
3. `wasm-build` — install `wasm-pack`, run `wasm-pack build codeviz-wasm --target web`
4. `wasm-size` — assert WASM bundle < 3MB, fail CI if exceeded

## release.yml Requirements
Trigger: push of tag `v*`.
Jobs:
1. Build Linux binary: `cargo build --release` → upload `codeviz` artifact
2. Build macOS binary (macos-latest) → upload `codeviz-macos` artifact
3. Create GitHub Release with both binaries attached and auto-generated release notes


═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- Use stable Rust 2021 edition. Do NOT use unstable features like `let_chains`.
- Always run `cargo test --all` and `cargo clippy --all -- -D warnings` before completing the task.
