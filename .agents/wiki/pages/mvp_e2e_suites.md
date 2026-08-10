---
title: MVP Full-Stack E2E Test Suites
tags: [e2e, testing, playwright, surrealdb, cli]
source_count: 1
---

# MVP Full-Stack E2E Test Suites

To validate CodeViz for each MVP release (v1, v2, v3), we utilize a unified, zero-mocking Full-Stack E2E Testing Strategy.

## Key Concepts
- **Zero Mocking**: Tests must execute the real compiled Rust CLI (`codeviz run`) against real files, and interact with the real Next.js UI via Playwright. No API or internal mock responses are permitted.
- **SurrealDB Seeding**: The in-memory database used for Playwright tests must be seeded with test user accounts and topological graph data using `seed.surql` to simulate the required environments (e.g., Enterprise single-tenant vs Multi-tenant teams).
- **Comprehensive Validation**: E2E tests validate that UI buttons correctly trigger backend MCP or CLI commands, and that the resulting data correctly persists to SurrealDB and flows back to the React Flow frontend.
