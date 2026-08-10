# TASK: T58 — MVP v2 Full E2E Test Suite

Please implement the MVP v2 E2E tests for CodeViz.

1. **Read Spec**: `docs/specs/features/mvp_e2e_suites.md`
2. **Context**: We need comprehensive Playwright E2E tests for the MVP v2 features (Team workspaces, Architecture Drift Alerts, Onboarding Walkthroughs).
3. **Execution**:
   - Update `codeviz-web/seed.surql` to add multi-user team scenarios.
   - Create Playwright tests in `codeviz-web/e2e/mvp2.spec.ts`.
   - The test must validate Team RBAC, trigger drift alerts via the CLI, and ensure the UI reflects the changes.
   - Run `npm run test:e2e` to ensure the new tests pass.
