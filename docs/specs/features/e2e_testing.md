# E2E Testing Strategy

## Objective
Implement a robust End-to-End (E2E) testing framework that validates the entire CodeViz stack, from the backend Rust parsers to the Next.js frontend UI.

## Architecture

### 1. CLI E2E Tests (Backend/Core)
- **Framework**: Rust `assert_cmd` and `predicates`.
- **Location**: `codeviz-cli/tests/e2e/`.
- **Workflow**: 
  - The test sets up a dummy fixture repository.
  - The compiled `codeviz` binary is executed against the fixture.
  - The resulting output (JSON/Mermaid) is validated against snapshot files to ensure parser correctness and CLI flag routing.

### 2. MCP Server E2E Tests (Integration)
- **Framework**: Python `pytest` + `mcp-sdk`.
- **Location**: `codeviz-mcp/tests/e2e/`.
- **Workflow**:
  - The test spins up `codeviz-mcp` as a background process.
  - Validates JSON-RPC standard communication over `stdio`.
  - Asserts that tools like `add_language_support` correctly register and return the expected graph formats.

### 3. Full-Stack UI Tests (Playwright)
- **Framework**: Playwright.
- **Location**: `codeviz-web/tests/e2e/`.
- **Workflow**:
  - Pre-requisite: The Rust backend must be compiled.
  - Playwright spins up the Next.js frontend alongside the `codeviz-mcp` backend process.
  - Verifies that loading the dashboard successfully fetches the graph, visualizes React Flow nodes, and opens the CFG Side Panel when a node is clicked.
  - Validates PGLite (in-memory) integration for auth and team workspaces (Zero Mocking).
