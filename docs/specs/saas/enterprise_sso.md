# Spec: Enterprise SSO & Org Management (Phase 15)

## Overview
Enterprise customers require Single Sign-On (SSO) and organization-level user
management so that all engineers are automatically provisioned from their Identity
Provider (IdP) without manual invitations.

---

## SSO Protocols Supported
- **SAML 2.0:** For enterprises using Okta, Azure AD, OneLogin, Ping Identity.
- **OIDC (OpenID Connect):** For enterprises using Google Workspace, Auth0, custom providers.

## SSO Provider Implementation
- Use **WorkOS** as the SSO abstraction layer.
- WorkOS handles all SAML/OIDC complexity and provides a single SDK.
- Admin configures their IdP in the CodeViz Org Settings page by pasting their SSO metadata URL.
- WorkOS redirects users to their corporate IdP and returns a normalized user profile.

---

## SCIM User Provisioning
- Enterprise plan supports **SCIM 2.0** for automated user lifecycle management.
- When an engineer joins the company's IdP group, they are automatically added to the CodeViz Workspace.
- When they leave, their access is automatically revoked.

## Database Schema

### `organizations` table
```sql
CREATE TABLE organizations (
  id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name          TEXT NOT NULL,
  slug          TEXT UNIQUE NOT NULL,
  workos_org_id TEXT UNIQUE,   -- WorkOS Organization ID for SSO
  scim_token    TEXT,          -- SCIM bearer token
  plan          TEXT NOT NULL DEFAULT 'enterprise',
  created_at    TIMESTAMPTZ DEFAULT now()
);
```

### `audit_logs` table
```sql
CREATE TABLE audit_logs (
  id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  org_id       UUID REFERENCES organizations(id),
  actor_id     UUID REFERENCES users(id),
  action       TEXT NOT NULL,  -- e.g., 'member.invited', 'view.deleted', 'sso.login'
  resource     TEXT,
  metadata     JSONB,
  created_at   TIMESTAMPTZ DEFAULT now()
);
```

---

## Enterprise Admin UI Pages

| Route | Description |
|---|---|
| `/org/[slug]/settings/sso` | Configure SSO: paste IdP metadata, test connection |
| `/org/[slug]/settings/scim` | SCIM token management |
| `/org/[slug]/settings/members` | View all members, roles, last login |
| `/org/[slug]/settings/audit-log` | Full audit log with filters |
| `/org/[slug]/settings/billing` | Plan details, invoice history |

---

## Pricing Gates
- SSO configuration page only accessible if `organization.plan === 'enterprise'`.
- Attempting to configure SSO on Pro plan shows an upgrade prompt.
- Audit log only accessible on Enterprise plan.

---

## Acceptance Criteria
- [ ] Enterprise admin can configure SSO via WorkOS.
- [ ] Users can log in via their corporate IdP (SSO).
- [ ] First SSO login auto-provisions the user in the `users` table (JIT provisioning).
- [ ] SCIM token can be generated and used to auto-add/remove users.
- [ ] All sensitive actions are logged in `audit_logs`.
- [ ] SSO and SCIM settings pages are gated behind Enterprise plan check.
