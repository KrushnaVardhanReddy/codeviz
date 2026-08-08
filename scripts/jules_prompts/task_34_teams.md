TASK: T34 — Team Workspaces & Repo Groups

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement Team Workspaces: a shared environment where multiple engineers can
view, annotate, and save CodeViz graphs together. Includes Repo Groups and
Saved Views features.

Files to Create/Modify:
- `codeviz-web/app/w/[slug]/page.tsx` (workspace home)
- `codeviz-web/app/w/[slug]/repos/[repo]/page.tsx` (repo graph viewer)
- `codeviz-web/app/w/[slug]/repos/[repo]/views/[view]/page.tsx` (saved view)
- `codeviz-web/app/w/[slug]/settings/page.tsx` (workspace settings)
- `codeviz-web/app/w/[slug]/settings/members/page.tsx` (member management)
- `codeviz-web/components/SaveViewModal.tsx`
- `codeviz-web/components/AnnotationTooltip.tsx`
- `codeviz-web/lib/supabase/workspaces.ts` (DB queries for workspaces)

Spec (READ ONLY — implement from it, never edit):
  docs/specs/saas/teams.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- CONTEXT: All workspace routes must check that the authenticated user is a member of that workspace. Return 403 if not.
- Pro plan must enforce a 5-member limit. Show an upgrade prompt if exceeded.
- Saved Views must store the full React Flow canvas state (zoom, pan, which nodes are expanded).
- Shareable links must work without login for 'viewer' access (read-only).
- Write unit tests for the workspace membership check logic in `workspaces.ts`.
- Ensure `npm run build` passes without errors.

═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- Use Supabase Row Level Security (RLS) to enforce workspace membership at the database level. This is the most secure approach.
- To get the React Flow canvas state for saving: call `useReactFlow().toObject()` which returns `{ nodes, edges, viewport }`. Store this JSON as `canvas_state` in `saved_views`.
- To restore a Saved View: call `useReactFlow().setViewport(canvasState.viewport)` and set the nodes/edges from the stored state.
- For the 5-member Pro limit: query `count(*)` from `workspace_members` before inserting a new member and compare against the plan limit.
- Use Supabase Realtime subscriptions to show live annotation updates when multiple team members are viewing the same graph simultaneously.
