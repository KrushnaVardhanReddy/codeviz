# Spec: Team Workspaces & Repo Groups (Phase 14)

## Overview
Pro and Enterprise users can create **Workspaces** — shared environments where
multiple engineers can view, annotate, and save CodeViz graphs together.
Each Workspace contains **Repo Groups** (collections of repositories) and
**Saved Views** (bookmarked graph states).

---

## Key Concepts

### Workspace
- A shared namespace owned by one user (the "admin").
- Pro plan: up to 5 members. Enterprise: unlimited.
- Has a slug (e.g., `acme-corp`) for URL routing: `/w/acme-corp/`.

### Repo Group
- A collection of repositories linked to a Workspace.
- Each repo maps to a `CodeGraph` JSON stored in Supabase Storage.
- Graphs are re-generated on-demand or via the GitHub Actions integration (Task 23).

### Saved View
- A snapshot of the React Flow canvas state: zoom level, pan position, and
  which nodes are expanded/collapsed.
- Stored as a JSON blob linked to a Workspace and Repo Group.
- Shareable via a unique URL: `/w/acme-corp/repos/my-api/views/auth-flow`.

### Annotations
- Team members can attach text notes to individual nodes.
- Notes are stored in the `annotations` table and displayed as a tooltip on the node.

---

## Database Schema

### `workspaces` table
```sql
CREATE TABLE workspaces (
  id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  slug       TEXT UNIQUE NOT NULL,
  name       TEXT NOT NULL,
  owner_id   UUID REFERENCES users(id),
  plan       TEXT NOT NULL DEFAULT 'pro',
  created_at TIMESTAMPTZ DEFAULT now()
);
```

### `workspace_members` table
```sql
CREATE TABLE workspace_members (
  workspace_id UUID REFERENCES workspaces(id) ON DELETE CASCADE,
  user_id      UUID REFERENCES users(id) ON DELETE CASCADE,
  role         TEXT NOT NULL DEFAULT 'viewer', -- 'admin' | 'editor' | 'viewer'
  PRIMARY KEY (workspace_id, user_id)
);
```

### `repo_groups` table
```sql
CREATE TABLE repo_groups (
  id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  workspace_id  UUID REFERENCES workspaces(id) ON DELETE CASCADE,
  name          TEXT NOT NULL,
  github_repos  JSONB,         -- array of { owner, repo } objects
  graph_json    JSONB,         -- cached CodeGraph JSON
  updated_at    TIMESTAMPTZ DEFAULT now()
);
```

### `saved_views` table
```sql
CREATE TABLE saved_views (
  id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  repo_group_id  UUID REFERENCES repo_groups(id) ON DELETE CASCADE,
  name           TEXT NOT NULL,
  canvas_state   JSONB,  -- { zoom, pan, expandedNodes[] }
  created_by     UUID REFERENCES users(id),
  created_at     TIMESTAMPTZ DEFAULT now()
);
```

### `annotations` table
```sql
CREATE TABLE annotations (
  id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  view_id     UUID REFERENCES saved_views(id) ON DELETE CASCADE,
  node_id     TEXT NOT NULL,   -- CodeGraph node ID
  content     TEXT NOT NULL,
  author_id   UUID REFERENCES users(id),
  created_at  TIMESTAMPTZ DEFAULT now()
);
```

---

## UI Pages

| Route | Description |
|---|---|
| `/w/[slug]` | Workspace home: list of Repo Groups |
| `/w/[slug]/repos/[repo]` | Graph viewer for a specific Repo Group |
| `/w/[slug]/repos/[repo]/views/[view]` | A specific Saved View |
| `/w/[slug]/settings` | Workspace settings: members, billing |
| `/w/[slug]/settings/members` | Invite members, manage roles |

---

## Acceptance Criteria
- [ ] User can create a Workspace with a unique slug.
- [ ] Workspace admin can invite members by email.
- [ ] Members can view and navigate the graphs in the Workspace.
- [ ] Editors can save a canvas state as a named Saved View.
- [ ] Saved Views are accessible via a shareable URL.
- [ ] Annotations can be added to any node in a Saved View.
- [ ] Pro plan enforces a 5-member limit on the workspace.
