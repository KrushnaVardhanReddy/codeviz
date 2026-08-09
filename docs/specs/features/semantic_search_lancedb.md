# Spec: Semantic Code Search with LanceDB (Phase 19)

## Overview

Adds opt-in, natural-language code search to the CodeViz SaaS web app,
powered by an embedded LanceDB vector index built on top of the `CodeGraph` IR.

This feature is intentionally **web-app only**. The offline CLI is untouched.
It requires an authenticated user session (Phase 13 Auth) and an active
Team Workspace (Phase 14) to be enabled.

---

## Motivation

The existing MCP tools (`get_callers`, `get_callees`, `explain_path`) perform
**exact** graph lookups. This is ideal for precise queries from an LLM with
known symbol names. However, human users often don't know the exact symbol name.
They want to type:
- *"Show me everything related to authentication"*
- *"Which modules handle caching?"*
- *"Find all the error handling code"*

LanceDB enables this by indexing each graph node as a vector embedding, then
performing an approximate nearest-neighbor (ANN) similarity search at query time.

---

## Prerequisites (must be merged before this task starts)

- **Task 33** (Auth): User session required to gate API key usage.
- **Task 34** (Teams & Workspaces): The index is scoped per-workspace, stored in Supabase Storage.
- **Phase 18** (Advanced Analysis): `CodeGraph` should be fully enriched with PageRank scores, health scores, etc. — these become additional metadata fields on each indexed node for richer results.

---

## Architecture

```
User types query in web UI
        │
        ▼
POST /api/search  { "workspace_id": "...", "query": "authentication flow" }
        │
        ▼
  Embed query text via OpenAI `text-embedding-3-small`
        │
        ▼
  LanceDB ANN search over workspace's node embedding table
        │
        ▼
  Return top-K matching Node IDs + similarity scores
        │
        ▼
  Web app highlights those nodes in the React Flow graph
```

---

## Data Model

### LanceDB Table: `node_embeddings`

| Column         | Type           | Description                                  |
|----------------|----------------|----------------------------------------------|
| `id`           | `string`       | Node.id (e.g. `src/auth.rs::verify_token`)   |
| `workspace_id` | `string`       | Scopes the index to a workspace              |
| `label`        | `string`       | Node.label (human-readable symbol name)      |
| `kind`         | `string`       | NodeKind serialized (e.g. `Function`)        |
| `file_path`    | `string`       | Node.file_path                               |
| `text`         | `string`       | The full text sent to the embedding model    |
| `vector`       | `vector(1536)` | Embedding from `text-embedding-3-small`      |
| `pagerank`     | `float`        | Optional — from Phase 18 PageRank scores     |
| `health_score` | `float`        | Optional — from Phase 18 Code Health scores  |

### Embedding Text Strategy

The `text` field (what gets embedded) is constructed as:
```
{kind}: {label}
file: {file_path}
callers: {comma-separated caller labels, up to 5}
callees: {comma-separated callee labels, up to 5}
```
This gives the embedding model structural context beyond just the symbol name.

---

## API Routes (Next.js)

### `POST /api/search/index`
Triggers re-indexing of a workspace's `CodeGraph`.
- **Auth**: Required (workspace member).
- **Body**: `{ "workspace_id": string, "graph": CodeGraph }`
- **Action**: Embeds all nodes in batches of 100, upserts into LanceDB table.
- **Response**: `{ "indexed_count": number, "duration_ms": number }`

### `POST /api/search/query`
Performs a semantic search.
- **Auth**: Required (workspace member).
- **Body**: `{ "workspace_id": string, "query": string, "limit": number? }`
- **Action**: Embeds the query, runs ANN search, returns top-K node IDs.
- **Response**: `{ "results": [{ "node_id": string, "score": float, "label": string, "file_path": string }] }`

---

## Web UI Changes

### `codeviz-web/components/SemanticSearchBar.tsx`
- A floating search bar component (keyboard shortcut: `Cmd+K` / `Ctrl+K`).
- Debounced — fires the query API after 400ms of inactivity.
- Displays results as a dropdown list; clicking a result pans the React Flow
  graph to and highlights the matching node.

### `codeviz-web/app/w/[slug]/repos/[repo]/page.tsx`
- Add `<SemanticSearchBar workspaceId={...} />` to the repo graph viewer.
- On result selection, call `useReactFlow().setCenter(node.x, node.y, { zoom: 1.5 })`.

---

## Constraints

- **Opt-in only**: Indexing must be manually triggered. Do NOT auto-index on every page load.
- **No CLI changes**: This feature is strictly web-app only. The Rust binary must not gain any LanceDB or embedding dependencies.
- **Embedding model is pluggable**: Read from env var `EMBEDDING_MODEL` (default: `text-embedding-3-small`).
- **Cost guardrail**: Warn user before re-indexing if `node_count > 5000`.
- **Privacy**: The `text` field sent to the embedding API must never include raw source code — only symbol names, kinds, and file paths.
- `npm run build` must pass without errors.
- Write unit tests for the embedding text construction logic.

---

## Files to Create/Modify

- `codeviz-web/app/api/search/index/route.ts` [NEW]
- `codeviz-web/app/api/search/query/route.ts` [NEW]
- `codeviz-web/components/SemanticSearchBar.tsx` [NEW]
- `codeviz-web/lib/lancedb.ts` [NEW] — LanceDB client wrapper
- `codeviz-web/lib/embeddings.ts` [NEW] — Embedding model wrapper
- `codeviz-web/app/w/[slug]/repos/[repo]/page.tsx` [MODIFY] — add search bar
- `package.json` [MODIFY] — add `@lancedb/lancedb` and `openai` dependencies

---

## Acceptance Criteria

- Indexing a `CodeGraph` with 100 nodes completes in under 10 seconds.
- A query for *"authentication"* on a repo containing an `auth.rs` module returns that module in the top 3 results.
- The `SemanticSearchBar` correctly pans the React Flow graph to the selected node.
- `/api/search/query` returns 403 if the user is not a workspace member.
- Re-indexing the same workspace upserts (does not duplicate) embeddings.
