TASK: T35 — Enterprise SSO & Audit Logs

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement Enterprise SSO via WorkOS (SAML 2.0 / OIDC) and an Audit Log system.
Enterprise admins configure SSO from the Org Settings page. All sensitive actions
are logged in the `audit_logs` table.

Files to Create/Modify:
- `codeviz-web/app/org/[slug]/settings/sso/page.tsx`
- `codeviz-web/app/org/[slug]/settings/audit-log/page.tsx`
- `codeviz-web/app/org/[slug]/settings/members/page.tsx`
- `codeviz-web/app/api/auth/sso/route.ts` (WorkOS SSO callback)
- `codeviz-web/lib/workos.ts` (WorkOS SDK wrapper)
- `codeviz-web/lib/audit.ts` (log action helper)

Spec (READ ONLY — implement from it, never edit):
  docs/specs/saas/enterprise_sso.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- CONTEXT: SSO and Audit Log pages must be gated: check `organization.plan === 'enterprise'` before rendering. Show an upgrade CTA otherwise.
- Use the WorkOS Node SDK (`@workos-inc/node`).
- On first SSO login, auto-provision the user in the `users` table if they don't exist (JIT provisioning).
- Every sensitive action (member invited, view deleted, SSO login, SCIM token generated) MUST call the `logAuditEvent()` helper.
- Write unit tests for the `logAuditEvent()` helper.

═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- Install: `npm install @workos-inc/node`.
- WorkOS SSO flow:
  1. Admin clicks "Configure SSO" → redirect to `workos.sso.getAuthorizationURL(...)`.
  2. User logs in at their IdP.
  3. IdP redirects back to `/api/auth/sso?code=...`.
  4. Exchange the code: `workos.sso.getProfileAndToken({ code })`.
  5. Use the returned profile to find or create the user in Supabase.
- For the Audit Log UI, use a simple server-side rendered table with pagination (10 rows per page). Add filter dropdowns for `action` type and date range.
- Store `WORKOS_API_KEY` and `WORKOS_CLIENT_ID` in `.env.local`.
- The `logAuditEvent(orgId, actorId, action, resource, metadata)` helper should simply `INSERT INTO audit_logs` — keep it synchronous and non-blocking (fire and forget).
