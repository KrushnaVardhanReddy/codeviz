# Spec: Authentication & User Management (Phase 13)

## Overview
CodeViz SaaS requires user authentication so that graphs, saved views, and team
workspaces can be persisted and shared. This spec covers the Auth layer for the
Next.js web app.

## Technology Stack
- **Auth Library:** NextAuth.js v5 (Auth.js)
- **Database:** Supabase (Postgres) for user and session storage
- **OAuth Providers (Free + Pro):** GitHub OAuth, Google OAuth
- **SSO Provider (Enterprise only):** SAML 2.0 / OIDC via Auth0 or WorkOS

---

## Authentication Flows

### 1. GitHub OAuth (Primary)
- User clicks "Sign in with GitHub"
- GitHub redirects back with OAuth token
- We store: `github_id`, `username`, `email`, `avatar_url`
- Scope required: `read:user`, `user:email`

### 2. Google OAuth
- Same flow as GitHub. Scope: `profile email`

### 3. Enterprise SSO (SAML / OIDC)
- Available on Enterprise plan only
- Admin configures SSO in Org Settings
- Users are auto-provisioned on first SSO login (JIT provisioning)
- Mapped fields: `email`, `name`, `groups` (for team assignment)

---

## Database Schema

### `users` table
```sql
CREATE TABLE users (
  id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  email       TEXT UNIQUE NOT NULL,
  name        TEXT,
  avatar_url  TEXT,
  github_id   TEXT UNIQUE,
  google_id   TEXT UNIQUE,
  plan        TEXT NOT NULL DEFAULT 'free', -- 'free' | 'pro' | 'enterprise'
  created_at  TIMESTAMPTZ DEFAULT now()
);
```

### `sessions` table (managed by NextAuth.js)
```sql
CREATE TABLE sessions (
  id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id       UUID REFERENCES users(id) ON DELETE CASCADE,
  token         TEXT UNIQUE NOT NULL,
  expires_at    TIMESTAMPTZ NOT NULL
);
```

---

## Route Protection
- `/app/*` routes: require authenticated session. Redirect to `/login` if not.
- `/api/*` routes: validate session token in middleware.
- Public routes: `/`, `/pricing`, `/docs`, `/login`.

---

## Acceptance Criteria
- [ ] User can sign in with GitHub OAuth.
- [ ] User can sign in with Google OAuth.
- [ ] Session is persisted across page refreshes.
- [ ] `/app` redirects to `/login` if unauthenticated.
- [ ] `users` table is created and populated on first login.
- [ ] User avatar and name appear in the top navbar after login.
