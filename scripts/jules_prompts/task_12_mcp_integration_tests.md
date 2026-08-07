TASK: T12 — MCP Integration Tests

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
1. Spawn `codeviz serve --mcp` as a subprocess
2. Send JSON-RPC requests via stdin, read responses from stdout
3. Test all 6 tools against the fixture files per the acceptance criteria in `docs/specs/07_mcp_protocol.md`
4. Test error handling: unknown tool → `-32601`, invalid params → `-32602`
5. Create `docs/mcp_config.md` with Claude Desktop, Cursor, and Continue.dev snippets from `docs/specs/07_mcp_protocol.md`

Files to Modify/Create:


Spec (READ ONLY — implement from it, never edit):
  docs/specs/07_mcp_protocol.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- Write comprehensive unit tests:

- No unwraps or panics in core parsing logic. Return Result.
- Ensure 'cargo clippy --all -- -D warnings' and 'cargo test --all' pass cleanly.
