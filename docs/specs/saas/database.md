# SaaS Database Architecture

## Overview
CodeViz utilizes a robust, real-time database architecture built on **SurrealDB** for production environments, ensuring robust capabilities for upcoming phases.

To test locally without network dependencies, E2E tests should utilize SurrealDB's in-memory mode, populated with seed data.

## Local E2E Testing Strategy (Playwright)
1. **In-Memory Mode**: Playwright spins up `surreal start memory` with a `seed.surql` file.
2. **Framework**: We use **Playwright** (`npm run test:e2e`) as the end-to-end framework.
3. **Official Auth.js Adapter**: We use `@auth/surrealdb-adapter` which connects to the local memory instance seamlessly during E2E.

## Implementation Requirements
- Initialize `@auth/surrealdb-adapter` and `surrealdb`.
- Create `codeviz-web/playwright.config.ts` to spin up SurrealDB alongside Next.js.
- Ensure the connection logic connects correctly based on `SURREALDB_URL` fallback `http://127.0.0.1:8000/rpc` during tests.
