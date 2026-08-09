TASK: T17 — Full Stack E2E Validation Suite (CLI, MCP, Playwright)

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement a tri-layered E2E testing framework to validate CodeViz end-to-end. Dogfood the CodeViz application on its own repository!

Files to Create/Modify:
- `codeviz-cli/tests/e2e_test.rs`
- `codeviz-mcp/tests/e2e_test.py`
- `codeviz-web/tests/e2e/dashboard.spec.ts`
- `codeviz-web/playwright.config.ts`

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/e2e_testing.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- DOGFOODING: The CLI E2E test (`codeviz-cli/tests/e2e_test.rs`) must execute `cargo run --bin codeviz -- run --path .` against the actual CodeViz root directory to parse itself! Assert that it successfully parses Rust and TypeScript files.
- MCP Python Test: Use `pytest` to spin up the `codeviz-mcp` binary via stdio, send `tools/list`, and assert `add_language_support` is present.
- Playwright: Use `@playwright/test`. Spin up the Next.js server locally and assert that the graph canvas renders correctly and the CFG Side Panel opens when an interactive element is clicked.
- Do NOT use mock data for the database if testing actual API routes, but do not attempt to use PGLite as a proxy for the Supabase SDK.

═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- Use the `assert_cmd` crate in `codeviz-cli` for spawning the CLI binary and asserting stdout/stderr.
- Use `subprocess.Popen` in Python to interface with the MCP server stdio.
- Ensure the Next.js frontend has proper `data-testid` attributes on the React Flow nodes and the `DetailPanel` so Playwright can click and assert visibility.
