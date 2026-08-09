TASK: T33B — Auth DB Adapter & Playwright E2E

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Integrate the NextAuth setup (from T33A) with Supabase to store user profiles,
and configure Playwright + PGLite for zero-mock local E2E testing.

Files to Create/Modify:
- `codeviz-web/lib/db.ts` (Supabase client with PGLite local proxy)
- `codeviz-web/playwright.config.ts` (Playwright E2E configuration)
- `codeviz-web/package.json` (add test:e2e script)

Spec (READ ONLY — implement from it, never edit):
  docs/specs/saas/auth.md
  docs/specs/saas/database.md

═══════════════════════════════════════════════════════════════
CRITICAL CONTEXT & GUARDRAILS (READ CAREFULLY)
═══════════════════════════════════════════════════════════════
- DIRECTORY STRUCTURE: Operate STRICTLY in the root folders of `codeviz-web`. DO NOT use or recreate a `src/` directory.
- This task assumes T33A (Auth Core) is already merged.

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- Use Supabase for the production database adapter.
- For local E2E tests, you MUST configure `@electric-sql/pglite` as a local, in-memory PostgreSQL instance and use Playwright for tests (`npm run test:e2e`).
- Add basic Playwright tests verifying the login flow UI.

═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- Install: `npm install @auth/supabase-adapter @electric-sql/pglite playwright`.
- Store `SUPABASE_URL`, `SUPABASE_SERVICE_ROLE_KEY` in `.env.local`.
- Use the Supabase adapter from `@auth/supabase-adapter` to auto-manage the `users` and `sessions` tables.
