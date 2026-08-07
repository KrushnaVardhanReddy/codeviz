TASK: T24 — npm WASM Package

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement npm packaging per the spec:
- Build with `wasm-pack build --target bundler`
- Correct TypeScript types for all 3 exported functions
- Publish workflow triggered on `v*` tags using `NPM_TOKEN` secret
- CDN usage example in README

Files to Modify/Create:


Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/npm_package.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- Write comprehensive unit tests:

- No unwraps or panics in core parsing logic. Return Result.
- Ensure 'cargo clippy --all -- -D warnings' and 'cargo test --all' pass cleanly.
