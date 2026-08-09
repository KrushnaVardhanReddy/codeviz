TASK: T33A — Auth Core: GitHub & Google OAuth (NextAuth.js)

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Add authentication to the CodeViz Next.js web app using NextAuth.js v5 (Auth.js)
with GitHub and Google OAuth providers. 

Files to Create/Modify:
- `codeviz-web/auth.ts` (NextAuth configuration)
- `codeviz-web/middleware.ts` (route protection)
- `codeviz-web/app/login/page.tsx`
- `codeviz-web/app/api/auth/[...nextauth]/route.ts`
- `codeviz-web/components/UserNav.tsx` (avatar + sign out in top navbar)

Spec (READ ONLY — implement from it, never edit):
  docs/specs/saas/auth.md

═══════════════════════════════════════════════════════════════
CRITICAL CONTEXT & GUARDRAILS (READ CAREFULLY)
═══════════════════════════════════════════════════════════════
- DIRECTORY STRUCTURE: Operate STRICTLY in the root `app/`, `components/`, and `lib/` folders of `codeviz-web`. DO NOT use, create, or recreate a `src/` directory. Next.js will crash with routing conflicts if both exist.
- GIT DIFFS: Ensure that `codeviz-web/node_modules/` is properly ignored by Git. If it accidentally gets tracked, run `git rm -r --cached node_modules/` to prevent massive PR diffs.

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- Use NextAuth.js v5 (Auth.js) — NOT the older v4 API.
- All `/app/*` routes must be protected by the middleware.
- Public routes: `/`, `/login`, `/pricing`, `/docs`.
- The login page must have "Sign in with GitHub" and "Sign in with Google" buttons.
- Write unit tests for the middleware route protection logic.
- Do NOT worry about database adapters (Supabase/PGLite) or Playwright E2E tests yet. That will be handled in T33B.

═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- Install: `npm install next-auth@beta`.
- The NextAuth.js v5 config exports `handlers`, `auth`, `signIn`, `signOut` from `auth.ts`.
- Use `export const { handlers, auth } = NextAuth({ providers: [...] })` pattern.
- In `middleware.ts`, use the `auth` export: `export default auth((req) => { ... })`.
