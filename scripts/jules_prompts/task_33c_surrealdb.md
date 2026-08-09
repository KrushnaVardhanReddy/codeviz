# Task 33C — SurrealDB Migration (Auth Adapter + E2E)

## Context

Read the full spec before writing any code:
> `docs/specs/saas/task_33c_surrealdb.md`

This task migrates `codeviz-web/` from Supabase to SurrealDB. The goal is:
1. Replace `@auth/supabase-adapter` with `@auth/surrealdb-adapter` in the NextAuth config.
2. Boot SurrealDB in-memory inside Playwright's `webServer` config so E2E tests run against a real database (no more JWT-only mocks).
3. Remove the `isE2E` bypass flag entirely from `auth.ts`.

This is a web-only task. No Rust crates are modified.

---

## Pre-requisites

Before writing any code, install the SurrealDB CLI binary into the sandbox:

```bash
curl -sSf https://install.surrealdb.com | sh
surreal version   # Confirm it's installed
```

---

## Step-by-Step Instructions

### Step 1: Update npm dependencies

```bash
cd codeviz-web
npm uninstall @auth/supabase-adapter @supabase/supabase-js
npm install @auth/surrealdb-adapter surrealdb
```

### Step 2: Create `codeviz-web/lib/surrealdb.ts`

Create a SurrealDB connection singleton:

```typescript
import Surreal from "surrealdb";

const db = new Surreal();

export const clientPromise: Promise<Surreal> = (async () => {
  const url    = process.env.SURREALDB_URL  ?? "http://127.0.0.1:8000/rpc";
  const user   = process.env.SURREALDB_USER ?? "root";
  const pass   = process.env.SURREALDB_PASS ?? "root";
  const ns     = process.env.SURREALDB_NS   ?? "codeviz";
  const dbName = process.env.SURREALDB_DB   ?? "main";

  await db.connect(url, {
    namespace: ns,
    database: dbName,
    auth: { username: user, password: pass },
  });
  return db;
})();
```

### Step 3: Rewrite `codeviz-web/auth.ts`

Replace the file entirely with:

```typescript
import NextAuth from "next-auth"
import GitHub from "next-auth/providers/github"
import Google from "next-auth/providers/google"
import { SurrealDBAdapter } from "@auth/surrealdb-adapter"
import { clientPromise } from "./lib/surrealdb"

export const { handlers, auth, signIn, signOut } = NextAuth({
  secret: process.env.AUTH_SECRET,
  providers: [GitHub, Google],
  adapter: SurrealDBAdapter(clientPromise),
})
```

**IMPORTANT**: Remove ALL references to `isE2E`, `Credentials` provider, and `SupabaseAdapter`. They must not exist in the final file.

### Step 4: Create `codeviz-web/seed.surql`

Create this file at the root of `codeviz-web/`:

```surql
-- Auth.js schema for SurrealDB
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD email ON user TYPE string ASSERT $value != NONE;
DEFINE FIELD emailVerified ON user TYPE option<datetime>;
DEFINE FIELD image ON user TYPE option<string>;
DEFINE INDEX email ON user COLUMNS email UNIQUE;

DEFINE TABLE session SCHEMAFULL;
DEFINE FIELD expires ON session TYPE datetime;
DEFINE FIELD sessionToken ON session TYPE string;
DEFINE FIELD userId ON session TYPE record<user>;
DEFINE INDEX sessionToken ON session COLUMNS sessionToken UNIQUE;

DEFINE TABLE account SCHEMAFULL;
DEFINE FIELD userId ON account TYPE record<user>;
DEFINE FIELD type ON account TYPE string;
DEFINE FIELD provider ON account TYPE string;
DEFINE FIELD providerAccountId ON account TYPE string;
DEFINE INDEX provider_account ON account COLUMNS provider, providerAccountId UNIQUE;

DEFINE TABLE verification_token SCHEMAFULL;
DEFINE FIELD identifier ON verification_token TYPE string;
DEFINE FIELD token ON verification_token TYPE string;
DEFINE FIELD expires ON verification_token TYPE datetime;

-- Test seed data
CREATE user:testuser CONTENT {
  name: "Test User",
  email: "testuser@example.com",
  emailVerified: time::now(),
  image: null
};
```

### Step 5: Rewrite `codeviz-web/playwright.config.ts`

Update the `webServer` config to start SurrealDB first, then Next.js:

```typescript
import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './e2e',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: 'html',
  use: {
    baseURL: 'http://localhost:3001',
    trace: 'on-first-retry',
  },
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
  ],
  webServer: [
    {
      command: 'surreal start --user root --pass root --ns codeviz --db main --import-file ./seed.surql memory',
      url: 'http://127.0.0.1:8000/health',
      reuseExistingServer: false,
      timeout: 10_000,
    },
    {
      command: 'npm run dev -- -p 3001',
      url: 'http://localhost:3001',
      reuseExistingServer: !process.env.CI,
      env: {
        SURREALDB_URL: 'http://127.0.0.1:8000/rpc',
        SURREALDB_USER: 'root',
        SURREALDB_PASS: 'root',
        SURREALDB_NS: 'codeviz',
        SURREALDB_DB: 'main',
      },
    },
  ],
});
```

### Step 6: Simplify `codeviz-web/e2e/login.spec.ts`

Remove the `Credentials` sign-in flow test (it no longer works since we removed that provider).
Keep only the redirect test:

```typescript
import { test, expect } from '@playwright/test';

test('unauthenticated users are redirected from /app', async ({ page }) => {
  await page.goto('/app');
  await page.waitForURL('**/api/auth/signin');
  expect(page.url()).toContain('/api/auth/signin');
});
```

### Step 7: Overwrite `docs/specs/saas/database.md`

Replace the file content to reflect SurrealDB as the canonical database. Remove all Supabase references.

---

## Verification

After implementing, run the following and confirm ALL pass:

```bash
# 1. Build check
cd codeviz-web && npm run build

# 2. E2E tests (SurrealDB starts automatically via playwright.config.ts)
npm run test:e2e

# 3. Rust tests (no changes expected, but always verify)
cd .. && cargo test --all
```

All tests must pass with exit code 0. Do NOT submit the PR if any test fails.

---

## Commit Message Format

```
jules: T33C — migrate auth adapter from Supabase to SurrealDB
```
