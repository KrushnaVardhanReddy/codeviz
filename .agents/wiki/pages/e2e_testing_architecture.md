---
title: E2E Testing Architecture
tags: [testing, e2e, playwright, cli, mcp]
source_count: 1
---

# E2E Testing Architecture

## Concept Overview
CodeViz adopts a tri-layered E2E testing approach to ensure full-stack reliability without relying entirely on mocked unit tests.

- **CLI E2E**: Validates the Rust binaries against real file system structures, ensuring parsers correctly emit valid Graph IR. Uses `assert_cmd`.
- **MCP Server E2E**: Validates the JSON-RPC interface and protocol compliance using a Python `pytest` suite that spawns the server via `stdio`.
- **Web UI E2E**: Validates the end-user experience (rendering React Flow nodes, interacting with CFG panels) using `Playwright`. It leverages PGLite for zero-mocking database environments.

This architecture ensures that breaking changes in the Rust core are caught before they impact the Next.js frontend or the MCP extension ecosystem.
