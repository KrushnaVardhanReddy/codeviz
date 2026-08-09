---
title: "SurrealDB-First Testing Pattern"
tags: ["surrealdb", "testing", "e2e", "architecture", "ai-agents", "pattern"]
source_count: 1
---

# SurrealDB-First Testing Pattern

## Summary

A universal meta-strategy for building projects where AI agents (Jules, Claude, etc.) can run full E2E integration tests with a **real database** — with zero Docker dependencies, zero network calls, and zero cleanup required.

## The Core Problem It Solves

AI agent sandboxes (Jules, GitHub Actions, Codespaces) are lightweight containers that do NOT support running Docker-in-Docker. This means any project using Supabase, PostgreSQL, MySQL, or MongoDB for its database **cannot run real database tests inside an AI agent sandbox**. Teams are forced to:
- Mock all database calls (reduces test confidence)
- Skip database tests entirely (dangerous for auth/data integrity)
- Run only against a remote staging database (slow, flaky, state-polluting)

## The Solution: SurrealDB In-Memory Mode

SurrealDB ships as a **single binary** that can be installed via `curl` and started in a zero-dependency in-memory mode in under 1 second.

```bash
# Install (one-time, ~10 seconds, no root required)
curl -sSf https://install.surrealdb.com | sh

# Start with seed data (0.2 seconds, auto-cleans on exit)
surreal start --user root --pass root \
  --ns myapp --db main \
  --import-file ./seed.surql \
  memory
```

- No Docker. No root access. No cleanup. No state leakage between runs.
- The database is automatically destroyed when the process exits.
- A `seed.surql` file (version-controlled in the repo) provides consistent, reproducible test data.

## The Playwright Integration Pattern

For Next.js applications using Playwright, SurrealDB starts before the Next.js dev server using the `webServer` array config:

```typescript
// playwright.config.ts
webServer: [
  {
    // SurrealDB starts FIRST, seeded with test data
    command: 'surreal start --user root --pass root --ns myapp --db main --import-file ./seed.surql memory',
    url: 'http://127.0.0.1:8000/health',
    reuseExistingServer: false,
    timeout: 10_000,
  },
  {
    // Next.js dev server starts SECOND, pointed at local SurrealDB
    command: 'npm run dev -- -p 3001',
    url: 'http://localhost:3001',
    env: { SURREALDB_URL: 'http://127.0.0.1:8000/rpc' },
  },
],
```

## Auth.js (NextAuth) Integration

SurrealDB has an **official Auth.js adapter** (`@auth/surrealdb-adapter`). This means session management, OAuth users, and account linking all persist to SurrealDB with zero custom code:

```typescript
import { SurrealDBAdapter } from "@auth/surrealdb-adapter"
import { clientPromise } from "./lib/surrealdb"

export const { handlers, auth } = NextAuth({
  adapter: SurrealDBAdapter(clientPromise),
  providers: [GitHub, Google],
})
```

In E2E test environments, the adapter connects to the local in-memory SurrealDB instead of the cloud — **no code changes needed, just environment variables**.

## The `seed.surql` Pattern

All test data lives in a single declarative SurrealQL file at the repo root. This file:
1. Defines the schema (tables, fields, indexes, constraints)
2. Seeds initial test data (users, sessions, etc.)
3. Is version-controlled alongside the application code

Example minimal auth schema:
```surql
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD email ON user TYPE string ASSERT $value != NONE;
DEFINE INDEX email ON user COLUMNS email UNIQUE;

-- Seed test user
CREATE user:testuser CONTENT {
  name: "Test User",
  email: "testuser@example.com",
  emailVerified: time::now()
};
```

## Applicability

This pattern works for any project that needs a database, not just CodeViz:

| Project Type | Benefit |
|---|---|
| Next.js SaaS app | Full auth + data E2E tests in Jules/CI |
| Rust CLI tool | Integration tests with persistent state |
| Go backend service | Real DB tests, no Docker Compose |
| Mobile app backend | API tests with real seed data |
| Go-based BaaS (future project) | Native Rust+Go SDK, zero external deps |

## Comparison to Alternatives

| | **Supabase** | **PostgreSQL** | **SQLite (PocketBase)** | **SurrealDB** |
|---|---|---|---|---|
| Docker required | ✅ Yes (7 containers!) | ✅ Yes | ❌ No | ❌ No |
| In-memory mode | ❌ No | ❌ No | ✅ Yes | ✅ Yes |
| Seed via file | ❌ SQL migrations only | ❌ SQL migrations only | ❌ No | ✅ `.surql` file |
| Auth.js adapter | ✅ Official | ❌ None | ❌ None | ✅ Official |
| AI agent friendly | ❌ No | ❌ No | ✅ Yes | ✅ Yes |
| Free cloud tier | ✅ 500MB | ❌ Self-hosted | ❌ Self-hosted | ✅ 1GB |
| Rust native SDK | ❌ HTTP only | ⚠️ via sqlx | ❌ No | ✅ First-class |

## Applied In

- **CodeViz T33C**: Migrated from Supabase to SurrealDB for auth and E2E testing.

## Source

- Spec: `docs/specs/saas/task_33c_surrealdb.md`
- SurrealDB docs: https://surrealdb.com/docs/surrealdb/introduction/start
- Auth.js SurrealDB adapter: https://authjs.dev/getting-started/adapters/surrealdb
