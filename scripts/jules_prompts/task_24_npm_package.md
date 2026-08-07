# Jules Task 24 — npm WASM Package

## Spec
Read `docs/specs/features/npm_package.md` before writing any code.

## Files to Create/Modify
- `codeviz-wasm/package.json`
- `.github/workflows/npm-publish.yml`
- `codeviz-wasm/README.md` (npm-focused usage guide)

## Requirements
Implement npm packaging per the spec:
- Build with `wasm-pack build --target bundler`
- Correct TypeScript types for all 3 exported functions
- Publish workflow triggered on `v*` tags using `NPM_TOKEN` secret
- CDN usage example in README

## Tests
Add an integration test that runs `wasm-pack test --node` to verify the
exported `parse()` function works in a Node.js environment.
