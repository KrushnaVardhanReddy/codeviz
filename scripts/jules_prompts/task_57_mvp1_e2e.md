# TASK: T57 — MVP v1 Full E2E Test Suite

Please implement the MVP v1 E2E tests for CodeViz.

1. **Read Spec**: `docs/specs/features/mvp_e2e_suites.md`
2. **Context**: We need comprehensive Playwright E2E tests for the MVP v1 features (Code Playground, Call Path Explorer, etc.).
3. **Execution**:
   - Update `codeviz-web/seed.surql` to ensure the required mock data and test users for MVP v1 are present.
   - Create Playwright tests in `codeviz-web/e2e/mvp1.spec.ts`.
   - The test must interact with the Next.js UI, ensure the Rust CLI is spawned (or WASM fallback used), and validate UI changes without mocking API calls.
   - Run `npm run test:e2e` to ensure the new tests pass.
