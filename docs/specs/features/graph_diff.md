# Spec: Graph Diff Mode (`codeviz diff`)

## Purpose
Show what architectural changes a PR introduces by comparing the CodeGraph
between two git refs. A killer feature for code review.

---

## CLI
```
codeviz diff --base <ref> --head <ref> --path <dir> [--diagram module|call|class]
```
`<ref>` can be a branch name, commit SHA, or tag.

---

## Algorithm
1. Checkout `--base` ref into a temp directory (using `git archive` — no working tree pollution).
2. Parse `--base` sources → `base_graph: CodeGraph`.
3. Parse `--head` sources (current working tree or checkout) → `head_graph: CodeGraph`.
4. Compute delta:
   - `added_nodes` = nodes in head but not in base (matched by `Node.id`)
   - `removed_nodes` = nodes in base but not in head
   - `added_edges` = edges in head but not in base
   - `removed_edges` = edges in base but not in head

---

## Output Formats

### Human-readable (default)
```
📊 Architecture diff: main → feature/mcp

  ➕ Added nodes (3):
     codeviz_mcp::server::McpServer
     codeviz_mcp::tools::get_callers
     codeviz_mcp::tools::get_callees

  ➖ Removed nodes (0): none

  ➕ Added edges (2):
     codeviz_core → codeviz_mcp [Imports]
     codeviz_cli → codeviz_mcp [Imports]
```

### Mermaid (with `--format mermaid`)
Renders a diagram where:
- Added nodes/edges are annotated with `:::added` CSS class
- Removed nodes/edges are annotated with `:::removed`

### JSON (with `--format json`)
```json
{
  "added_nodes": [...],
  "removed_nodes": [...],
  "added_edges": [...],
  "removed_edges": [...]
}
```

---

## Acceptance Criteria
- Adding a new file produces `added_nodes` for its symbols.
- Deleting a function produces `removed_nodes`.
- Renaming a function produces one removed + one added node.
- `--base` and `--head` being identical produces empty diff, exit 0.
- Works without any local git checkout (uses `git archive` for base).
