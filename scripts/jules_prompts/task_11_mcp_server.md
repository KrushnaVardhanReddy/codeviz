# Jules Task 11 — MCP Server: Core & Tool Definitions

## Spec
Read `docs/specs/07_mcp_protocol.md` before writing any code.

## Files to Modify/Create
- `codeviz-mcp/src/lib.rs`
- `codeviz-mcp/src/server.rs`
- `codeviz-mcp/src/tools.rs`
- `codeviz-mcp/Cargo.toml`

## Requirements
Implement the MCP server per `docs/specs/07_mcp_protocol.md`:
- All 6 tools with their JSON Schema definitions and Rust handlers
- stdio JSON-RPC 2.0 transport
- `max_nodes` cap (default 200)
- All errors returned as JSON-RPC error objects (never panic)
- Wire into CLI: `codeviz serve --mcp [--port N]`

## Unit Tests
- Test each tool handler with a known mock `CodeGraph`
- Test unknown tool name returns error code `-32601`
- Test invalid params returns error code `-32602`
