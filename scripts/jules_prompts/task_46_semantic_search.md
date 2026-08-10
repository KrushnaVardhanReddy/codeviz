TASK: T46 — Semantic Code Search with LanceDB

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement opt-in, natural-language semantic search over the `CodeGraph` using
LanceDB vector embeddings (OpenAI `text-embedding-3-small`). This is a web-app-only
feature scoped per Team Workspace. The CLI and Rust binary must remain unchanged.

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/semantic_search_lancedb.md

═══════════════════════════════════════════════════════════════
BACKGROUND — WHAT ALREADY EXISTS
═══════════════════════════════════════════════════════════════

- Team Workspaces exist from T34 (workspace members, repo_groups).
- Authentication is handled by Auth.js from T33.
- The graph viewer is at `codeviz-web/app/w/[slug]/repos/[repo]/page.tsx`.
- The `GraphCanvas` uses React Flow, exposed via `useReactFlow()`.
- Phase 18 enriches nodes with `health_score` and `pagerank` (from T43/T44).

PREREQUISITES:
  This task REQUIRES T33 (Auth), T34 (Teams), T43 (PageRank), and T44 (Health Scores)
  to be merged first. Do NOT start this task until those are complete.

═══════════════════════════════════════════════════════════════
DELIVERABLES
═══════════════════════════════════════════════════════════════

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. MODIFY: codeviz-web/package.json
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Add dependencies:
- `@lancedb/lancedb` (LanceDB client)
- `openai` (for embedding API calls)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
2. CREATE: codeviz-web/lib/lancedb.ts
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
A LanceDB client singleton wrapper.
- Opens (or creates) a LanceDB database in a persistent location.
- Exposes `upsert(workspaceId, records[])` and `search(workspaceId, vector, limit)` methods.
- The `node_embeddings` table schema:
  `id`, `workspace_id`, `label`, `kind`, `file_path`, `text`, `vector(1536)`, `pagerank`, `health_score`

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
3. CREATE: codeviz-web/lib/embeddings.ts
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
An OpenAI embedding wrapper.
- Model: read from `process.env.EMBEDDING_MODEL` (default `text-embedding-3-small`).
- Function: `embedTexts(texts: string[]) -> number[][]`.
- Constructs the embedding text using the strategy from the spec:
  ```
  {kind}: {label}
  file: {file_path}
  callers: {up to 5 caller labels, comma-separated}
  callees: {up to 5 callee labels, comma-separated}
  ```
- IMPORTANT: Never send raw source code — only symbol names, kinds, and file paths.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
4. CREATE: API Routes
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

`codeviz-web/app/api/search/index/route.ts` [POST]:
- Auth: Required (must be workspace member).
- Body: `{ "workspace_id": string, "graph": CodeGraph }`
- Action: Embed all nodes in batches of 100, upsert into LanceDB.
- Response: `{ "indexed_count": number, "duration_ms": number }`
- Warn if `node_count > 5000` before proceeding.

`codeviz-web/app/api/search/query/route.ts` [POST]:
- Auth: Required (must be workspace member).
- Body: `{ "workspace_id": string, "query": string, "limit": number? }` (default limit: 10)
- Action: Embed the query, run ANN search in LanceDB.
- Response: `{ "results": [{ "node_id": string, "score": float, "label": string, "file_path": string }] }`
- Return 403 if user is not a workspace member.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
5. CREATE: codeviz-web/components/SemanticSearchBar.tsx
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
A floating search bar component:
- Triggered by keyboard shortcut `Cmd+K` / `Ctrl+K`.
- Debounced — fires the `/api/search/query` API after 400ms of inactivity.
- Displays results as a dropdown list with file path and score.
- On result click: calls `useReactFlow().setCenter(node.x, node.y, { zoom: 1.5 })` to
  pan the graph to the matching node and highlights it.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
6. MODIFY: codeviz-web/app/w/[slug]/repos/[repo]/page.tsx
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
- Add `<SemanticSearchBar workspaceId={...} />` to the graph viewer.
- Add an "Index for Search" button (opt-in) that calls `POST /api/search/index`.

═══════════════════════════════════════════════════════════════
CRITICAL IMPLEMENTATION RULES
═══════════════════════════════════════════════════════════════
1. ZERO CLI CHANGES: Do not modify any Rust crate or add any Rust dependencies.
2. NEVER embed raw source code. Only symbol names, kinds, file paths.
3. Indexing is ALWAYS manual/opt-in. Do NOT auto-index on page load.
4. Use `process.env.OPENAI_API_KEY` for the embedding API. Do NOT hardcode keys.
5. Re-indexing must upsert (update) existing records, not duplicate them.
6. Write unit tests for the embedding text construction function.
7. Ensure `npm run build` passes without TypeScript errors.

Commit: "jules: T46 — Semantic code search with LanceDB and OpenAI embeddings"
Target branch: feat-t46-semantic-search
