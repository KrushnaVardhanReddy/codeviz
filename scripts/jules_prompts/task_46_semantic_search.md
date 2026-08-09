TASK: T46 — Semantic Code Search with LanceDB (Phase 19)

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Add opt-in natural-language code search to the CodeViz SaaS web app,
powered by LanceDB (embedded vector database) and OpenAI embeddings.
This is a web-app only feature — do NOT modify the Rust CLI or any
codeviz-* Rust crates.

Files to Create:
- `codeviz-web/app/api/search/index/route.ts`   (indexing endpoint)
- `codeviz-web/app/api/search/query/route.ts`   (search endpoint)
- `codeviz-web/components/SemanticSearchBar.tsx` (Cmd+K floating search UI)
- `codeviz-web/lib/lancedb.ts`                  (LanceDB client wrapper)
- `codeviz-web/lib/embeddings.ts`               (OpenAI embedding wrapper)

Files to Modify:
- `codeviz-web/app/w/[slug]/repos/[repo]/page.tsx` (add SemanticSearchBar)
- `codeviz-web/package.json`  (add `@lancedb/lancedb` and `openai`)

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/semantic_search_lancedb.md

═══════════════════════════════════════════════════════════════
PREREQUISITES — DO NOT START UNTIL THESE ARE MERGED
═══════════════════════════════════════════════════════════════
- Task 33 (Auth) must be merged first (session required for API auth)
- Task 34 (Teams & Workspaces) must be merged first (workspace_id scoping)
- Phase 18 (Advanced Analysis) should be complete (for pagerank/health enrichment)

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- NO CLI CHANGES. This is web-app only. Zero changes to any Rust crate.
- The embedding model name must be read from the `EMBEDDING_MODEL` env var.
  Default: `text-embedding-3-small`.
- The `text` field sent to the OpenAI API must contain ONLY:
    {kind}: {label}
    file: {file_path}
    callers: {up to 5 caller labels}
    callees: {up to 5 callee labels}
  NEVER include raw source code in the embedding text.
- Indexing is opt-in. Do NOT trigger auto-indexing on page load.
- Show a cost-warning modal if node_count > 5000 before indexing.
- All workspace routes must 401/403 if the user is not authenticated
  or not a member of the target workspace.
- Ensure `npm run build` passes without errors.
- Write unit tests for the embedding text construction function in `lib/embeddings.ts`.

═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- Use `@lancedb/lancedb` npm package. Connect to a local LanceDB
  directory at `process.env.LANCEDB_PATH` (default: `.lancedb`).
- Table name convention: `nodes_{workspace_id}` (one table per workspace).
- Use `table.add()` for initial index; `table.update()` or `table.merge_insert()`
  for re-indexing (upsert to avoid duplicates keyed on `id + workspace_id`).
- For the SemanticSearchBar, use the `cmdk` npm package for the Cmd+K palette UI.
- Debounce the search input by 400ms before firing the API call.
- On result click, use `useReactFlow().setCenter(x, y, { zoom: 1.5, duration: 500 })`
  and briefly highlight the matched node with a distinct border color.
