# Spec: MCP Protocol

## Purpose
The MCP adapter exposes CodeViz as a tool server to any MCP-compatible AI assistant
(Claude Desktop, Cursor, Continue.dev, etc.) via JSON-RPC 2.0 over stdio.

---

## MCP Spec Version
Implements: MCP 2025-06-18
Transport: stdio (default), HTTP/SSE (optional via `--port`)

---

## Tools (6 total)

### `get_module_graph`
Returns the full module dependency graph for a directory.
```json
Input:  { "path": string, "max_nodes": number? }
Output: { "graph": CodeGraph, "mermaid": string }
```

### `get_callers`
Returns all nodes that call a given function.
```json
Input:  { "fn_name": string, "path": string }
Output: { "callers": Node[] }
```

### `get_callees`
Returns all nodes called by a given function.
```json
Input:  { "fn_name": string, "path": string }
Output: { "callees": Node[] }
```

### `get_class_hierarchy`
Returns the full inheritance tree as a Mermaid classDiagram.
```json
Input:  { "path": string }
Output: { "mermaid": string }
```

### `find_entry_points`
Returns all nodes with no incoming `Calls` edges (i.e., nothing calls them).
```json
Input:  { "path": string }
Output: { "nodes": Node[] }
```

### `explain_path`
Returns the shortest dependency path between two named nodes.
```json
Input:  { "from": string, "to": string, "path": string }
Output: { "nodes": Node[], "exists": boolean }
```

---

## Constraints
- `max_nodes` default: 200. If exceeded, truncate and set `"truncated": true` in response.
- All errors must be returned as JSON-RPC error objects:
  - Unknown tool → error code `-32601`
  - Invalid params → error code `-32602`
  - Internal error → error code `-32603`
- Server must never panic — all errors must be caught and returned as JSON-RPC errors.

---

## Client Config Snippets

### Claude Desktop
```json
// ~/.config/claude/claude_desktop_config.json
{
  "mcpServers": {
    "codeviz": {
      "command": "codeviz",
      "args": ["serve", "--mcp"]
    }
  }
}
```

### Cursor
```json
// .cursor/mcp.json
{
  "mcpServers": {
    "codeviz": {
      "command": "codeviz",
      "args": ["serve", "--mcp"]
    }
  }
}
```

---

## Acceptance Criteria
- Sending a `tools/list` request returns all 6 tool definitions with correct JSON schemas.
- Each tool returns valid JSON on success.
- Sending an unknown tool name returns error code `-32601`.
- Server stays alive and processes multiple sequential requests without restarting.
