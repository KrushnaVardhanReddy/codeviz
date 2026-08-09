# SaaS Database Architecture

## Overview
CodeViz utilizes a robust, real-time database architecture built on **Supabase** (PostgreSQL) for production environments, ensuring robust RBAC and team workspace capabilities (Phase 13-16). 

To guarantee a completely local, zero-mocking development and testing environment, CodeViz leverages **PGLite** (a WASM-based local PostgreSQL instance) for all automated E2E tests.

## Local E2E Testing Strategy (Playwright + PGLite)
1. **Zero Mocking**: Tests hit an actual, local PostgreSQL engine (PGLite) embedded in the test runner memory, preventing the need to mock network calls or database SDK logic.
2. **Framework**: We use **Playwright** (`npm run test:e2e`) as the end-to-end framework. 
3. **Seeding**: The Playwright `test.beforeEach` hooks automatically instantiate a fresh PGLite instance and seed the SQL schema.
4. **Supabase Client Proxy**: The application's Supabase client is configured to proxy local requests to the PGLite driver when the `E2E_TEST` environment variable is active.

## Implementation Requirements
- Initialize `@supabase/supabase-js` and `@electric-sql/pglite` dependencies.
- Implement the client router in `codeviz-web/lib/db.ts` to switch between cloud Supabase and in-memory PGLite.
- Create `codeviz-web/playwright.config.ts` for E2E setups.
