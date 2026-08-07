# Jules Task 12 — MCP Integration Tests

## Spec
Read `docs/specs/07_mcp_protocol.md` before writing any code.

## Files to Create
- `codeviz-mcp/tests/integration_test.rs`
- `codeviz-mcp/tests/fixtures/sample_python.py`
- `codeviz-mcp/tests/fixtures/sample_typescript.ts`
- `docs/mcp_config.md` (client configuration guide)

## Requirements
1. Spawn `codeviz serve --mcp` as a subprocess
2. Send JSON-RPC requests via stdin, read responses from stdout
3. Test all 6 tools against the fixture files per the acceptance criteria in `docs/specs/07_mcp_protocol.md`
4. Test error handling: unknown tool → `-32601`, invalid params → `-32602`
5. Create `docs/mcp_config.md` with Claude Desktop, Cursor, and Continue.dev snippets from `docs/specs/07_mcp_protocol.md`
