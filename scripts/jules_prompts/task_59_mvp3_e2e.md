# TASK: T59 — MVP v3 Full E2E Test Suite

Please implement the MVP v3 E2E tests for CodeViz.

1. **Read Spec**: `docs/specs/features/mvp_e2e_suites.md`
2. **Context**: We need comprehensive Playwright E2E tests for the MVP v3 Enterprise features (OpenTelemetry Trace Overlay, Multi-Repo Graph, SBOM Export).
3. **Execution**:
   - Update `codeviz-web/seed.surql` to add multi-repo enterprise topology, mock OTEL traces, and SBOM metadata.
   - Create Playwright tests in `codeviz-web/e2e/mvp3.spec.ts`.
   - The test must interact with the Next.js UI, ensure cross-repo graph rendering and SBOM export capabilities via the CLI work properly without mocked APIs.
   - Run `npm run test:e2e` to ensure the new tests pass.
