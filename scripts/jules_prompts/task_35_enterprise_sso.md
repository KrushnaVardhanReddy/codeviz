TASK: T35 — Enterprise SSO & Org Management

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement Enterprise Single Sign-On (SAML/OIDC) and SCIM user provisioning
via WorkOS. Add organization-level auditing and SSO settings pages for
enterprise admins.

Spec (READ ONLY — implement from it, never edit):
  docs/specs/saas/enterprise_sso.md

═══════════════════════════════════════════════════════════════
BACKGROUND — WHAT ALREADY EXISTS
═══════════════════════════════════════════════════════════════

- Teams schema exists in `codeviz-web/seed.surql` (from T34).
- Next.js application uses Auth.js for basic OAuth and session management.

═══════════════════════════════════════════════════════════════
DELIVERABLES
═══════════════════════════════════════════════════════════════

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. MODIFY: codeviz-web/seed.surql
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Add the schema for `organizations` and `audit_logs` (translated from SQL to SurrealDB schema).
- Link `organizations` to WorkOS IDs and SCIM tokens.
- Add an `audit_logs` table.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
2. INTEGRATE: WorkOS SDK
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Install `@workos-inc/node` in `codeviz-web`.
Create an API route `codeviz-web/app/api/auth/sso/route.ts` to handle WorkOS redirects
and callbacks.
Implement Just-In-Time (JIT) user provisioning: when a user logs in via SSO, automatically
create their user record if it doesn't exist, and add them to the associated organization.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
3. CREATE: Enterprise Admin UI
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Create the settings pages defined in the spec:
- `/org/[slug]/settings/sso`
- `/org/[slug]/settings/scim`
- `/org/[slug]/settings/members`
- `/org/[slug]/settings/audit-log`

Ensure these pages are gated: they should only be accessible if `organization.plan === 'enterprise'`.
Show an upgrade prompt if a non-enterprise org attempts to access them.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
4. IMPLEMENT: Audit Logging API
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Create a utility function `logAuditAction(orgId, actorId, action, resource, metadata)`
that writes to the `audit_logs` table.

═══════════════════════════════════════════════════════════════
CRITICAL IMPLEMENTATION RULES
═══════════════════════════════════════════════════════════════
1. Do not hardcode WorkOS API keys. Use `process.env.WORKOS_API_KEY` and `WORKOS_CLIENT_ID`.
2. Do NOT break existing standard auth (GitHub/Google OAuth).
3. Ensure all sensitive DB actions (like logging in via SSO) write to the audit log.
4. Run `npm run build` to ensure no TypeScript compilation errors.

Commit: "jules: T35 — Enterprise SSO and SCIM via WorkOS"
Target branch: feat-t35-enterprise-sso
