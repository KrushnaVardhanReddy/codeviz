# CodeViz LLM Wiki Rules

CodeViz uses a persistent, compounding LLM Wiki stored in `.agents/wiki/`. 
The LLM (you, Jules, or OpenCode) is the maintainer of this wiki.

## Directory Structure
- **`.agents/wiki/raw/`**: Immutable raw sources (articles, PDFs, notes).
- **`.agents/wiki/assets/`**: Images and media.
- **`.agents/wiki/pages/`**: LLM-authored markdown files (entities, concepts).
- **`.agents/wiki/index.md`**: Catalog of all pages in the wiki.
- **`.agents/wiki/log.md`**: Append-only chronological log of operations.

> **Note:** The `docs/specs/` directory in the root is also treated as a **raw source**.

## Protocol: Ingest
When instructed to ingest a source (either a new file in `raw/` or a spec in `docs/specs/`), follow these steps:
1. **Read**: Read the raw source thoroughly.
2. **Page Creation/Update**: 
   - Create new concept/entity pages in `.agents/wiki/pages/` for any major ideas.
   - Update existing pages if the new source provides new details or contradicts old ones.
   - Every page should include YAML frontmatter with `title`, `tags`, and `source_count`.
3. **Index**: Update `.agents/wiki/index.md` with links to any newly created pages, organized by category.
4. **Log**: Append a chronological entry to `.agents/wiki/log.md` with the format `## [YYYY-MM-DD] ingest | Source Title`.
5. **Commit**: (If running autonomously) Commit the changes to the repository.

## Protocol: Query
When a user asks a deep architectural or conceptual question:
1. First, read `.agents/wiki/index.md` to discover relevant pages.
2. Read the relevant pages.
3. Synthesize the answer.
4. If your synthesis yields a valuable new insight, comparison, or connection, **save it as a new page in the wiki** and update the index/log (following the Ingest Protocol).

## Protocol: Lint
When instructed to "lint the wiki", you must:
1. Scan `.agents/wiki/pages/` for orphan pages (pages with no inbound links).
2. Look for concepts that are repeatedly mentioned but lack their own dedicated page.
3. Check for contradictions or stale claims.
4. Suggest actions to the user to fix these issues.
