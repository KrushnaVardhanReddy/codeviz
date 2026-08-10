TASK: T62 — Graph Rendering Stress Test

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement a graph rendering stress test to benchmark React Flow performance
for large repositories (>10,000 files). The test must generate a synthetic
CodeGraph JSON fixture and use Playwright to assert that the Time-To-Interactive
(TTI) on the graph canvas remains within acceptable limits.

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/qa_blind_spots.md (Section 3: Graph Rendering Stress Test)

═══════════════════════════════════════════════════════════════
BACKGROUND — WHAT ALREADY EXISTS (READ ONLY — do not modify unless specified)
═══════════════════════════════════════════════════════════════

EXISTING INFRASTRUCTURE:
  - Playwright is fully configured in `codeviz-web/playwright.config.ts`.
  - The Web UI renders graphs via React Flow in `codeviz-web/components/GraphCanvas.tsx` (using `data-testid="graph-canvas"`).
  - The IR schema is defined in `codeviz-core/src/graph.rs` (Node and Edge structs).

═══════════════════════════════════════════════════════════════
DELIVERABLES
═══════════════════════════════════════════════════════════════

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. CREATE: scripts/generate_stress_fixture.py
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Write a Python script that generates a synthetic `CodeGraph` JSON fixture.
- It must generate exactly 10,000 nodes and 20,000 edges.
- The output format must exactly match the serialized `CodeGraph` Rust struct:
  {
    "nodes": [ { "id": "...", "label": "...", "kind": "File", "file_path": "...", "is_public": true } ],
    "edges": [ { "source": "...", "target": "...", "kind": "Imports" } ],
    "meta": { "schema_version": "1.0", "generator": "stress_test" }
  }
- Save the output to `codeviz-web/e2e/fixtures/stress_graph.json`.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
2. CREATE: codeviz-web/e2e/stress.spec.ts
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Create a Playwright test file to benchmark the performance.

  test('React Flow canvas should render 10,000 nodes in under 5 seconds', async ({ page }) => {
    // 1. Load the fixture from `e2e/fixtures/stress_graph.json`.
    // 2. Start a performance timer.
    // 3. Navigate to a test page or mock the API response to serve the stress graph.
    //    (e.g., page.route('**/api/graph/**', route => route.fulfill({ json: stressGraph })))
    // 4. Wait for the graph canvas (data-testid="graph-canvas") to be visible.
    // 5. Wait for at least one `.react-flow__node` to appear in the DOM.
    // 6. Stop the timer.
    // 7. Assert that the elapsed time is less than 5000 milliseconds.
  });

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
3. VERIFY: Run the Stress Test
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Run `python3 scripts/generate_stress_fixture.py`.
Run `cd codeviz-web && npm run test:e2e -- stress.spec.ts`.

If the test fails because React Flow is too slow, DO NOT optimize React Flow in
this task. The goal of this task is just to build the benchmark. Leave the test
failing or skipped (test.skip) with a comment if it cannot meet the 5s deadline.

═══════════════════════════════════════════════════════════════
CRITICAL IMPLEMENTATION RULES
═══════════════════════════════════════════════════════════════
1. Do not modify existing tests or Rust crates.
2. The Python script must use only standard library modules (e.g. `json`).

Commit: "jules: T62 — Graph rendering stress test"
Target branch: feat-t62-stress-test
