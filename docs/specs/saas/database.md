# SaaS Database Architecture

## Overview
CodeViz utilizes a robust, real-time database architecture built on **Supabase** (PostgreSQL) for production environments, ensuring robust RBAC and team workspace capabilities (Phase 13-16). 

To test locally without network dependencies, E2E tests should bypass the DB adapter entirely and rely on NextAuth's mock sessions or JWT tokens.

## Local E2E Testing Strategy (Playwright)
1. **Mocked Sessions**: Tests interacting with protected routes should use a mocked NextAuth session (e.g. `CredentialsProvider` active only when `E2E_TEST=true`).
2. **Framework**: We use **Playwright** (`npm run test:e2e`) as the end-to-end framework. 
3. **No SDK Proxies**: Do NOT attempt to build a proxy between the Supabase JS SDK and local SQL instances (like PGLite), as this is brittle and complex.

## Implementation Requirements
- Initialize `@supabase/supabase-js`.
- Create `codeviz-web/playwright.config.ts` for E2E setups.
- Use `adapter: process.env.E2E_TEST ? undefined : SupabaseAdapter(...)` in `auth.ts` to bypass DB writes during testing.
