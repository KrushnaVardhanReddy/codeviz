TASK: T33 — Auth: GitHub & Google OAuth (NextAuth.js)

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Add authentication to the CodeViz Next.js web app using NextAuth.js v5 (Auth.js)
with GitHub and Google OAuth providers. Store user profiles in Supabase.

Files to Create/Modify:
- `codeviz-web/auth.ts` (NextAuth configuration)
- `codeviz-web/middleware.ts` (route protection)
- `codeviz-web/app/login/page.tsx`
- `codeviz-web/app/api/auth/[...nextauth]/route.ts`
- `codeviz-web/components/UserNav.tsx` (avatar + sign out in top navbar)
- `codeviz-web/lib/db.ts` (Supabase client)

Spec (READ ONLY — implement from it, never edit):
  docs/specs/saas/auth.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- CONTEXT: Use NextAuth.js v5 (Auth.js) — NOT the older v4 API. The v5 config file is `auth.ts` at the project root, not inside `pages/api/`.
- Use Supabase as the database adapter for storing sessions and users.
- All `/app/*` routes must be protected by the middleware.
- Public routes: `/`, `/login`, `/pricing`, `/docs`.
- The login page must have "Sign in with GitHub" and "Sign in with Google" buttons.
- Write unit tests for the middleware route protection logic.

═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- Install: `npm install next-auth@beta @auth/supabase-adapter`.
- The NextAuth.js v5 config exports `handlers`, `auth`, `signIn`, `signOut` from `auth.ts`.
- Use `export const { handlers, auth } = NextAuth({ providers: [...] })` pattern.
- In `middleware.ts`, use the `auth` export: `export default auth((req) => { ... })`.
- Store `NEXTAUTH_SECRET`, `GITHUB_CLIENT_ID`, `GITHUB_CLIENT_SECRET`, `GOOGLE_CLIENT_ID`, `GOOGLE_CLIENT_SECRET`, `SUPABASE_URL`, `SUPABASE_SERVICE_ROLE_KEY` in `.env.local`.
- Use the Supabase adapter from `@auth/supabase-adapter` to auto-manage the `users` and `sessions` tables.
