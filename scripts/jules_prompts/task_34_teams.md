TASK: T34 — Team Workspaces & Repo Groups

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement the backend architecture and UI scaffolding for Team Workspaces and Repo Groups.
This allows Pro and Enterprise users to create shared environments where multiple
engineers can view, annotate, and save CodeViz graphs together.

Spec (READ ONLY — implement from it, never edit):
  docs/specs/saas/teams.md

═══════════════════════════════════════════════════════════════
BACKGROUND — WHAT ALREADY EXISTS
═══════════════════════════════════════════════════════════════

- SurrealDB integration exists in `codeviz-web/seed.surql` with basic user/session models.
- Next.js web application is set up in `codeviz-web/app`.
- The `GraphCanvas` component can render a `CodeGraph` JSON blob.

═══════════════════════════════════════════════════════════════
DELIVERABLES
═══════════════════════════════════════════════════════════════

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. MODIFY: codeviz-web/seed.surql
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Add the database schema for workspaces, repo groups, saved views, and annotations.
Translate the SQL schema from the spec into SurrealDB schema format (`DEFINE TABLE`).

  - `workspaces`
  - `workspace_members`
  - `repo_groups`
  - `saved_views`
  - `annotations`

Add seed data for a workspace with some members and a repo group.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
2. CREATE/MODIFY: Next.js Routes
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Implement the required UI pages (they can be skeleton/scaffolding components that
fetch from the database):

- `codeviz-web/app/w/[slug]/page.tsx` — Workspace home (lists Repo Groups)
- `codeviz-web/app/w/[slug]/repos/[repo]/page.tsx` — Graph viewer for a specific Repo Group
- `codeviz-web/app/w/[slug]/repos/[repo]/views/[view]/page.tsx` — A specific Saved View
- `codeviz-web/app/w/[slug]/settings/page.tsx` — Workspace settings
- `codeviz-web/app/w/[slug]/settings/members/page.tsx` — Manage members

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
3. CREATE: API Routes for Workspace Management
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Create backend API routes in `codeviz-web/app/api/` (or Server Actions) to:
- Create a workspace.
- Invite members (enforce 5-member limit on 'pro' plan).
- Create a repo group.
- Save a view (canvas state).
- Add annotations to nodes.

═══════════════════════════════════════════════════════════════
CRITICAL IMPLEMENTATION RULES
═══════════════════════════════════════════════════════════════
1. Do NOT mock database calls. Use real SurrealDB queries.
2. The UI does not need to be perfectly styled, but the React components must wire up correctly to the database.
3. Ensure all Server Actions/APIs check authorization (user is logged in and belongs to workspace).
4. Run `npm run build` and `npm run test:e2e` to ensure no breakages.

Commit: "jules: T34 — Team Workspaces & Repo Groups"
Target branch: feat-t34-teams
