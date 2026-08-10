TASK: T58 — MVP v2 Full-Stack E2E Test Suite

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement a comprehensive, zero-mock Full-Stack E2E test suite for CodeViz
MVP v2. The suite must validate Team Features (RBAC, Architecture Drift Alerts,
Onboarding Walkthroughs) using a real SurrealDB in-memory instance and native
Rust CLI execution.

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/mvp_e2e_suites.md (MVP v2 section)

═══════════════════════════════════════════════════════════════
BACKGROUND — WHAT ALREADY EXISTS (READ ONLY — do not modify unless specified)
═══════════════════════════════════════════════════════════════

EXISTING E2E INFRASTRUCTURE:
  - Playwright is fully configured in `codeviz-web/playwright.config.ts`.
  - The MVP v1 suite exists in `codeviz-web/e2e/mvp1.spec.ts`.
  - The SurrealDB seed file is `codeviz-web/seed.surql`.

MVP v2 FEATURES TO TEST:
  - Architecture Drift Alerts: The CLI can evaluate drift rules and emit warnings.
  - Team Workspaces & RBAC: Users belong to Organizations. Graphs can be public or org-private.
  - Onboard Me: Auto-generated architecture walkthroughs.

═══════════════════════════════════════════════════════════════
DELIVERABLES
═══════════════════════════════════════════════════════════════

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. MODIFY: codeviz-web/seed.surql
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Append a "-- MVP v2 E2E test data" section AFTER the existing seed.
Add team workspace data, simulated git history, and drift alert rules:

  -- MVP v2 E2E test data
  DEFINE TABLE organization SCHEMAFULL;
  DEFINE FIELD name ON organization TYPE string;
  
  DEFINE TABLE org_member SCHEMAFULL;
  DEFINE FIELD orgId ON org_member TYPE record<organization>;
  DEFINE FIELD userId ON org_member TYPE record<user>;
  DEFINE FIELD role ON org_member TYPE string ASSERT $value IN ["admin", "member"];

  CREATE organization:acme CONTENT { name: "Acme Corp" };
  CREATE org_member:acme_testuser CONTENT {
      orgId: organization:acme,
      userId: user:testuser,
      role: "admin"
  };

  -- Create a second user outside the org for RBAC testing
  CREATE user:outsider CONTENT {
      name: "Outsider",
      email: "outsider@example.com",
      emailVerified: time::now()
  };
  CREATE session:outsider_session CONTENT {
      expires: "2030-01-01T00:00:00Z",
      sessionToken: "e2e-outsider-session",
      userId: user:outsider
  };

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
2. CREATE: codeviz-web/e2e/mvp2.spec.ts
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Create a new file with the following test cases:

--- Test 1: Team RBAC prevents unauthorized access ---
  test('should enforce RBAC on team graphs', async ({ browser }) => {
    // 1. Log in as 'user:outsider' using their session token.
    // 2. Navigate to a graph URL owned by 'organization:acme'.
    // 3. Assert the page displays a 403 Forbidden or "Not Found" error state.
    // 4. Log in as 'user:testuser' (org admin).
    // 5. Navigate to the same graph URL.
    // 6. Assert the graph canvas (data-testid="graph-canvas") successfully renders.
  });

--- Test 2: Architecture Drift Alerts via CLI ---
  test('should trigger a drift alert when CLI detects a violation', async () => {
    // 1. Create a temp directory with a codeviz.toml containing a drift rule:
    //    [rules]
    //    no_circular_deps = true
    // 2. Create source code that violates the rule (e.g. A imports B, B imports A).
    // 3. Spawn the Rust CLI: `codeviz run --path <tmpDir> --check`
    // 4. Assert: exitCode !== 0 (or specific failure code).
    // 5. Assert: stderr or stdout contains the drift alert warning text.
  });

--- Test 3: Onboard Me UI flows ---
  test('should render the Onboard Me walkthrough doc', async ({ page }) => {
    // 1. Navigate to 'http://localhost:3000/onboard' (or equivalent route).
    // 2. Wait for data-testid="onboard-doc" to be visible.
    // 3. Assert that the generated architecture summary text is present.
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
3. Test gracefully skips CLI steps if binary is missing.

Commit: "jules: T58 — MVP v2 Full-Stack E2E Test Suite"
Target branch: feat-t58-mvp2-e2e
