TASK: T59 — MVP v3 Full-Stack E2E Test Suite

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement a comprehensive, zero-mock Full-Stack E2E test suite for CodeViz
MVP v3. The suite must validate Enterprise Features (OpenTelemetry Trace Overlay,
Multi-Repo Graph, SBOM Export) using a real SurrealDB in-memory instance and native
Rust CLI execution.

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/mvp_e2e_suites.md (MVP v3 section)

═══════════════════════════════════════════════════════════════
BACKGROUND — WHAT ALREADY EXISTS (READ ONLY — do not modify unless specified)
═══════════════════════════════════════════════════════════════

EXISTING E2E INFRASTRUCTURE:
  - Playwright is fully configured in `codeviz-web/playwright.config.ts`.
  - The SurrealDB seed file is `codeviz-web/seed.surql`.

MVP v3 FEATURES TO TEST:
  - OpenTelemetry Trace Overlay: Visualizing execution paths over the architecture graph.
  - Multi-Repo Graph: Seeing dependencies across different repositories.
  - SBOM Export: Compliance requirement to export graph data to CycloneDX/SPDX.

═══════════════════════════════════════════════════════════════
DELIVERABLES
═══════════════════════════════════════════════════════════════

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. MODIFY: codeviz-web/seed.surql
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Append a "-- MVP v3 E2E test data" section AFTER the existing seed.
Add multi-repo enterprise topology, mock OTEL traces, and SBOM metadata:

  -- MVP v3 E2E test data
  DEFINE TABLE repository SCHEMAFULL;
  DEFINE FIELD name ON repository TYPE string;
  DEFINE FIELD url ON repository TYPE string;

  CREATE repository:backend CONTENT { name: "Backend API", url: "https://github.com/acme/backend" };
  CREATE repository:frontend CONTENT { name: "Frontend App", url: "https://github.com/acme/frontend" };

  -- Mock OTEL trace data
  DEFINE TABLE trace SCHEMAFULL;
  DEFINE FIELD trace_id ON trace TYPE string;
  DEFINE FIELD spans ON trace TYPE array;

  CREATE trace:test_trace CONTENT {
      trace_id: "trace-123",
      spans: [
          { service: "frontend", operation: "GET /api/data" },
          { service: "backend", operation: "query_db" }
      ]
  };

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
2. CREATE: codeviz-web/e2e/mvp3.spec.ts
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Create a new file with the following test cases:

--- Test 1: Multi-Repo Graph Rendering ---
  test('should render dependencies across multiple repositories', async ({ page }) => {
    // 1. Navigate to the multi-repo graph view (e.g., 'http://localhost:3000/enterprise/topology').
    // 2. Wait for the graph canvas to be visible.
    // 3. Assert that nodes from both 'Backend API' and 'Frontend App' are present in the DOM.
  });

--- Test 2: OTEL Trace Overlay ---
  test('should overlay an OTEL trace onto the graph', async ({ page }) => {
    // 1. Navigate to the graph view.
    // 2. Input a trace ID ("trace-123") into the trace search input (data-testid="trace-input").
    // 3. Click the "Load Trace" button (data-testid="load-trace-btn").
    // 4. Assert that the graph highlights the nodes involved in the trace (e.g. checks for a specific CSS class or style like stroke color).
  });

--- Test 3: SBOM Export via CLI ---
  test('should export SBOM via the Rust CLI', async () => {
    // 1. Create a temp directory with a minimal codebase.
    // 2. Spawn the Rust CLI: `codeviz export --format cyclonedx --path <tmpDir> --output sbom.json`
    // 3. Assert: exitCode === 0.
    // 4. Read `sbom.json` and assert that it contains the expected CycloneDX JSON schema fields.
  });

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
3. VERIFY: Run the E2E suite
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Run `cd codeviz-web && npm run test:e2e`. All tests must pass. Add missing
data-testids to the React UI if necessary.

═══════════════════════════════════════════════════════════════
CRITICAL IMPLEMENTATION RULES
═══════════════════════════════════════════════════════════════
1. ZERO MOCKING: Real CLI execution, real database.
2. Do not modify existing tests or Rust crates.

Commit: "jules: T59 — MVP v3 Full-Stack E2E Test Suite"
Target branch: feat-t59-mvp3-e2e
