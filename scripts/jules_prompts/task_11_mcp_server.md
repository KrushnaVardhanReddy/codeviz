TASK: T11 — MCP Server: Core & Tool Definitions

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement the MCP server per `docs/specs/07_mcp_protocol.md`:
- All 6 tools with their JSON Schema definitions and Rust handlers
- stdio JSON-RPC 2.0 transport
- `max_nodes` cap (default 200)
- All errors returned as JSON-RPC error objects (never panic)
- Wire into CLI: `codeviz serve --mcp [--port N]`

Files to Modify/Create:
- `codeviz-mcp/src/lib.rs`
- `codeviz-mcp/src/server.rs`
- `codeviz-mcp/src/tools.rs`
- `codeviz-mcp/Cargo.toml`

Spec (READ ONLY — implement from it, never edit):
  docs/specs/07_mcp_protocol.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- CONTEXT: We use JSON-RPC 2.0 over stdio. Use stable Rust 2021. Avoid `let_chains` or any nightly features.
- Write comprehensive unit tests:
- Test each tool handler with a known mock `CodeGraph`
- Test unknown tool name returns error code `-32601`
- Test invalid params returns error code `-32602`
- No unwraps or panics in core parsing logic. Return Result.
- Ensure 'cargo clippy --all -- -D warnings' and 'cargo test --all' pass cleanly.

═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- Use the `serde_json` crate heavily for RPC message parsing.
- You do NOT need to wait for all language parsers to be finished. Just use the `LanguageParser` registry trait.
- If testing locally, remember that `stdout` is used for the MCP protocol, so use `eprintln!` for any debugging logs to avoid breaking the JSON-RPC stream.
