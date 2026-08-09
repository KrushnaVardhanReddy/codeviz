# Task 33C — SurrealDB Migration: Replace Supabase Auth Adapter

## Overview

This task migrates the CodeViz SaaS backend from Supabase (PostgreSQL) to **SurrealDB Cloud** for production, while enabling **SurrealDB in-memory mode** for local development and all E2E test environments (Jules, CI, developer machines). This eliminates the primary blocker in our current E2E testing strategy: the inability to run the Supabase stack (7 Docker containers) inside lightweight AI agent sandboxes.

## Motivation

### Current Pain Points (Supabase)
1. **E2E Tests are Mocked**: Because Supabase cannot run inside Jules' sandboxes (requires Docker-in-Docker), we had to bypass the DB adapter entirely in T33B using `adapter: isE2E ? undefined : SupabaseAdapter(...)`. This means our E2E tests never test real database writes — they only test JWT sessions. This is a fundamental gap in our test coverage.
2. **Local Dev Complexity**: Every new developer must install Docker, run `supabase start`, wait 2-5 minutes for 7 containers to start, before they can even run the app locally.
3. **AI Agent Friction**: Jules and other AI agents repeatedly fail or waste session time fighting Docker/Supabase setup, causing tasks to take 2-3x longer than necessary.

### Why SurrealDB Fixes All Three
- **Single binary**: `curl -sSf https://install.surrealdb.com | sh` — downloaded and ready in ~10 seconds.
- **In-memory mode**: `surreal start memory` — starts in 0.2 seconds, auto-cleans on exit. No Docker, no cleanup.
- **Seed files**: Data seeded via a single `.surql` file — declarative, version-controlled, reproducible.
- **Official Auth.js adapter**: `@auth/surrealdb-adapter` exists and is officially maintained — no custom adapter needed.
- **Free cloud tier**: SurrealDB Cloud offers 1GB free — more than sufficient for MVP.
- **Rust-native**: First-class Rust SDK (`surrealdb` crate) aligns perfectly with our Rust backend for future features.

---

## Implementation Requirements

### 1. Install SurrealDB Adapter
```bash
cd codeviz-web
npm uninstall @auth/supabase-adapter @supabase/supabase-js
npm install @auth/surrealdb-adapter surrealdb
```

### 2. Create `codeviz-web/lib/surrealdb.ts`
A connection singleton that reads environment variables and connects to either the cloud or in-memory instance:

```typescript
import Surreal from "surrealdb";

const db = new Surreal();

export const clientPromise: Promise<Surreal> = (async () => {
  const url    = process.env.SURREALDB_URL  ?? "http://127.0.0.1:8000/rpc";
  const user   = process.env.SURREALDB_USER ?? "root";
  const pass   = process.env.SURREALDB_PASS ?? "root";
  const ns     = process.env.SURREALDB_NS   ?? "codeviz";
  const dbName = process.env.SURREALDB_DB   ?? "main";

  await db.connect(url, { namespace: ns, database: dbName, auth: { username: user, password: pass } });
  return db;
})();
```

### 3. Update `codeviz-web/auth.ts`
Replace the Supabase adapter with SurrealDB. Remove the `isE2E` flag entirely — in-memory DB handles E2E natively:

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

### 4. Create `codeviz-web/seed.surql`
A declarative SurrealQL seed file for E2E tests:

```surql
-- Schema
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

-- E2E seed user
CREATE user CONTENT {
  name: "Test User",
  email: "testuser@example.com",
  emailVerified: time::now(),
  image: null
};
```

### 5. Update `codeviz-web/playwright.config.ts`
Boot SurrealDB in-memory BEFORE the Next.js dev server using `webServer` array:

```typescript
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
    env: { SURREALDB_URL: 'http://127.0.0.1:8000/rpc' },
  },
],
```

### 6. Update `codeviz-web/e2e/login.spec.ts`
Remove the E2E `Credentials` provider workaround. The redirect test (unauthenticated → `/api/auth/signin`) remains unchanged.

### 7. Update `.env.local.example`
```bash
# SurrealDB Cloud (production)
SURREALDB_URL=https://your-instance.surrealdb.cloud/rpc
SURREALDB_USER=your_user
SURREALDB_PASS=your_pass
SURREALDB_NS=codeviz
SURREALDB_DB=main

# REMOVED (no longer needed):
# SUPABASE_URL=
# SUPABASE_SERVICE_ROLE_KEY=
```

### 8. Update `docs/specs/saas/database.md`
Replace all Supabase references with SurrealDB.

---

## Verification Steps

1. `npm run build` inside `codeviz-web/` — must complete with zero errors.
2. `npm run test:e2e` — all Playwright tests must pass with a REAL database (no JWT bypass).
3. `cargo test --all` — all Rust tests must still pass (no Rust changes in this task).
4. Confirm `@supabase/supabase-js` and `@auth/supabase-adapter` are absent from `package.json`.

---

## Notes
- SurrealDB binary must be installed first: `curl -sSf https://install.surrealdb.com | sh`
- `surreal start memory` does NOT require Docker or root access.
- The `isE2E` environment flag in `auth.ts` and the `Credentials` provider hack from T33B must be fully deleted.
- No Rust crates are changed in this task. It is purely a `codeviz-web/` migration.
