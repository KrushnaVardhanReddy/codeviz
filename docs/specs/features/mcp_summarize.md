# Spec: `summarize_architecture` MCP Tool (Phase 20)

## Overview
A new MCP tool that returns a **human-readable narrative summary** of a repository's
architecture. Unlike the existing exact-lookup tools, this tool is designed to give
an LLM (or a new engineer) an immediate high-level mental model of the codebase.

## Tool Definition

### `summarize_architecture`
```json
Input:  { "path": string }
Output: {
  "summary": string,
  "stats": {
    "total_nodes": number,
    "total_edges": number,
    "languages": string[],
    "entry_points": string[],
    "top_modules": string[],
    "circular_dep_count": number
  }
}
```

## Summary Generation Rules

The `summary` string must be generated from the `CodeGraph` deterministically
(no LLM required) using the following template:

```
This is a {language_list} codebase with {node_count} symbols across {file_count} files.

Entry points: {entry_point_labels}.
Most-imported modules: {top_5_by_in_degree}.
{if circular_deps} ⚠️ {n} circular dependencies detected. {/if}
{if health_score_available} Average code health score: {avg_score}/10. {/if}
```

## Files to Modify
- `codeviz-mcp/src/tools.rs` — add `summarize_architecture` handler
- `codeviz-core/src/lib.rs` — add `summarize()` method to `CodeGraph`

## Constraints
- No external API calls. Summary is generated purely from the `CodeGraph`.
- Must be deterministic: same graph → same output.
- Ensure `cargo test --all` and `cargo clippy --all` pass.
