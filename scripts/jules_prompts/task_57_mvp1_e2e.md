TASK: T57 — MVP v1 Full-Stack E2E Test Suite

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement a comprehensive, zero-mock Full-Stack E2E test suite for CodeViz
MVP v1. The suite must validate the complete feature stack — from the
Next.js web UI down to native Rust CLI execution — using a real SurrealDB
in-memory instance.

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/mvp_e2e_suites.md (MVP v1 section)

═══════════════════════════════════════════════════════════════
BACKGROUND — WHAT ALREADY EXISTS (READ ONLY — do not modify unless specified)
═══════════════════════════════════════════════════════════════

The following infrastructure is complete:

EXISTING E2E TESTS (do not modify these files — only add new files):
  codeviz-web/e2e/dashboard.spec.ts  — Tests graph canvas render and node click
  codeviz-web/e2e/login.spec.ts      — Tests login page visibility
  codeviz-web/e2e/playground.spec.ts — Tests the code playground page

EXISTING SEED FILE (MODIFY to add MVP v1 test data):
  codeviz-web/seed.surql
  - Currently seeds: user:testuser with email testuser@example.com
  - The SurrealDB Auth.js tables (user, session, account, verification_token) are all defined there.

PLAYWRIGHT CONFIG:
  codeviz-web/playwright.config.ts — Playwright is already configured.
  `npm run test:e2e` runs all tests in codeviz-web/e2e/

WEB UI FEATURES IMPLEMENTED (targets for test assertions):
  - /app page: The main graph canvas (data-testid="graph-canvas") renders a React Flow graph.
  - /playground page: The code playground (data-testid="playground-editor") allows source input.
  - Call Path Explorer: Triggered from the detail panel (data-testid="detail-panel") with a
    "Trace Call Path" button that opens the CallPathExplorer component in the UI.
  - CFG Side Panel: data-testid="cfg-panel" opens when a function node is clicked.

RUST CLI:
  The `codeviz` binary is built via `cargo build --release` and available at:
  `./target/release/codeviz` (from the workspace root).
  CLI usage for generating a graph to stdout:
    codeviz run --path <dir> --diagram module
  Exit code 0 on success. Writes Mermaid markdown to stdout or to a file if --output is given.

═══════════════════════════════════════════════════════════════
DELIVERABLES
═══════════════════════════════════════════════════════════════

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. MODIFY: codeviz-web/seed.surql
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Append a "-- MVP v1 E2E test data" section AFTER the existing seed.
DO NOT remove or alter the existing schema definitions or testuser record.

Add:
  -- MVP v1 E2E test data
  -- A second user for multi-user scenarios
  CREATE user:e2e_user CONTENT {
    name: "E2E Test User",
    email: "e2e@codeviz.dev",
    emailVerified: time::now(),
    image: null
  };

  -- A session for e2e_user so Playwright tests can be pre-authenticated
  CREATE session:e2e_session CONTENT {
    expires: "2030-01-01T00:00:00Z",
    sessionToken: "e2e-playwright-session-token",
    userId: user:e2e_user
  };


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
2. CREATE: codeviz-web/e2e/mvp1.spec.ts
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Create a new file with the following FOUR test cases:

--- Test 1: Rust CLI spawns and produces Mermaid output ---

  test('should spawn the Rust CLI and produce a valid Mermaid diagram', async () => {
    // 1. Create a temporary directory.
    // 2. Write a minimal codeviz.toml to it (empty config is fine: just `[project]`).
    // 3. Write a simple Python file (e.g., a single function definition) to the temp dir.
    // 4. Spawn the Rust CLI:
    //      const { stdout, stderr, exitCode } = await spawnSync(
    //        './target/release/codeviz',
    //        ['run', '--path', tmpDir, '--diagram', 'module'],
    //        { cwd: '<workspace root>' }
    //      )
    //    Use Node.js `child_process.spawnSync` or `execSync` inside the test.
    //    Import: import { execSync } from 'child_process'; import * as os from 'os'; import * as fs from 'fs';
    // 5. Assert: exitCode === 0
    // 6. Assert: stdout contains "graph TD" (Mermaid module graph header)
    // 7. Clean up the temporary directory.
  });

--- Test 2: Call Path Explorer opens in the Web UI ---

  test('should open the Call Path Explorer when Trace button is clicked', async ({ page }) => {
    // 1. Navigate to 'http://localhost:3000'
    // 2. Wait for data-testid="graph-canvas" to be visible.
    // 3. Click on the first available React Flow node: page.locator('.react-flow__node').first()
    // 4. Wait for data-testid="detail-panel" to be visible.
    // 5. Assert that a button containing text "Trace" or data-testid="trace-btn" is visible in the panel.
    // 6. Click the Trace button.
    // 7. Assert that data-testid="call-path-explorer" is visible (the CallPathExplorer component).
  });

--- Test 3: Code Playground parses source and renders a graph ---

  test('should render a graph from source code typed into the playground', async ({ page }) => {
    // 1. Navigate to 'http://localhost:3000/playground'
    // 2. Wait for data-testid="playground-editor" to be visible.
    // 3. Type a simple Python snippet into the editor textarea:
    //      "def hello():\n    pass\ndef world():\n    hello()"
    // 4. Click the "Analyze" or "Run" button (look for a button with text "Analyze" or similar).
    // 5. Wait for data-testid="graph-canvas" to become visible on the page.
    // 6. Assert that at least one `.react-flow__node` is rendered.
  });

--- Test 4: CFG side panel appears for a function node ---

  test('should show the CFG panel when a function node is clicked', async ({ page }) => {
    // 1. Navigate to 'http://localhost:3000'
    // 2. Wait for data-testid="graph-canvas" to be visible.
    // 3. Look for a React Flow node with data-type="Function":
    //      page.locator('.react-flow__node[data-type="Function"]').first()
    //    OR click the first available node and check if cfg-panel appears.
    // 4. Click the node.
    // 5. Assert that data-testid="cfg-panel" is visible.
  });


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
3. VERIFY: Run the full E2E suite and fix any failures
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

After creating the files, run:
  cd codeviz-web && npm run test:e2e

All existing tests (dashboard.spec.ts, login.spec.ts, playground.spec.ts) must
still pass. The four new tests in mvp1.spec.ts must also pass.
If any test fails due to a missing data-testid on a UI component, DO NOT modify
the test assertions — instead, add the missing data-testid to the relevant React
component in codeviz-web/components/.

═══════════════════════════════════════════════════════════════
CRITICAL IMPLEMENTATION RULES
═══════════════════════════════════════════════════════════════

1. ZERO MOCKING: Do not mock any API routes, database calls, or CLI execution.
   The Playwright tests must hit real running services.

2. The Rust CLI binary MUST be pre-built before the tests run. The test for CLI
   execution should skip gracefully (test.skip) if the binary does not exist at
   `./target/release/codeviz`, with a clear skip message.

3. Do NOT modify any existing E2E test files.

4. Do NOT modify any Rust crates.

5. Do NOT modify playwright.config.ts.

═══════════════════════════════════════════════════════════════
FILES LIST
═══════════════════════════════════════════════════════════════

FILES TO MODIFY:
  codeviz-web/seed.surql                (append MVP v1 test data)
  codeviz-web/components/               (add data-testid attributes ONLY if missing)

FILES TO CREATE:
  codeviz-web/e2e/mvp1.spec.ts         (four new E2E test cases)

FILES NOT TO TOUCH (READ-ONLY):
  codeviz-web/e2e/dashboard.spec.ts
  codeviz-web/e2e/login.spec.ts
  codeviz-web/e2e/playground.spec.ts
  codeviz-web/playwright.config.ts
  codeviz-web/next.config.ts
  codeviz-core/                         (all Rust crates)
  docs/specs/                           (all spec files)

Commit: "jules: T57 — MVP v1 Full-Stack E2E Test Suite"
Target branch: feat-t57-mvp1-e2e
