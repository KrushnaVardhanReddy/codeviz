---
title: "Semantic Code Search with LanceDB"
tags: ["lancedb", "vector-search", "embeddings", "phase-19", "saas", "web-ui"]
source_count: 1
---

# Semantic Code Search with LanceDB

## What It Is

An **opt-in** natural-language search feature for the CodeViz SaaS web app.
It converts each node in a `CodeGraph` into a vector embedding and stores
them in a LanceDB table, enabling approximate nearest-neighbor (ANN) search
when a user types a natural language query.

## Why LanceDB (and not Supabase pgvector)

LanceDB is embedded (no server) and written in Rust. In a future phase it
could be integrated into the CLI. For now it runs server-side in Next.js API
routes. Supabase pgvector was considered but adds network latency per query
and couples the feature to the cloud SaaS backend.

## Key Design Decisions

1. **Web-app only**: Zero changes to the Rust CLI or any `codeviz-*` crate.
2. **Opt-in indexing**: Never auto-indexed on page load. User triggers it.
3. **Privacy**: Only symbol names, kinds, and file paths are sent to the
   embedding model — never raw source code.
4. **Pluggable model**: `EMBEDDING_MODEL` env var (default: `text-embedding-3-small`).
5. **Per-workspace tables**: Table name = `nodes_{workspace_id}`.
6. **Cost guardrail**: Warning shown if `node_count > 5000` before indexing.

## Embedding Text Strategy

For each node, the embedding input is:
```
{kind}: {label}
file: {file_path}
callers: {up to 5 caller labels}
callees: {up to 5 callee labels}
```
This provides structural context (call relationships) alongside the symbol name.

## Data Model

LanceDB table `node_embeddings`:
- `id`, `workspace_id`, `label`, `kind`, `file_path`, `text` — metadata
- `vector(1536)` — embedding
- `pagerank`, `health_score` — optional enrichment from Phase 18

## API Routes

- `POST /api/search/index` — batch-embeds all nodes and upserts into LanceDB
- `POST /api/search/query` — embeds query, runs ANN search, returns top-K node IDs

## UI

`SemanticSearchBar.tsx`: floating `Cmd+K` palette (uses `cmdk` package).
On result click → `useReactFlow().setCenter(x, y, { zoom: 1.5 })` to pan graph.

## Prerequisites

This task (T46) must NOT start until:
- T33 (Auth) is merged
- T34 (Teams & Workspaces) is merged
- Phase 18 (Advanced Analysis) is complete

## Source Spec

`docs/specs/features/semantic_search_lancedb.md`
