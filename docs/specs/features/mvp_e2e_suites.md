# Spec: MVP Full-Stack E2E Test Suites

## Overview
To ensure production readiness for each major MVP release phase (v1, v2, v3), we require comprehensive, zero-mocking Full-Stack E2E Test Suites. These test suites must validate the entire flow: from the Next.js UI down to the native Rust CLI execution, using a real in-memory SurrealDB database seeded with test data.

## Constraints
1. **Zero Mocking**: No API routes, GraphQL resolvers, or Rust binary outputs may be mocked.
2. **SurrealDB Seeding**: Each test suite must start with a clean in-memory SurrealDB instance populated via a `seed.surql` script specific to the features being tested.
3. **Rust CLI Execution**: Tests must trigger real parsing execution via the compiled `codeviz run` binary in a temporary directory.
4. **Playwright Frontend**: Next.js pages and buttons must be tested interactively using Playwright.

## Test Suite Scopes

### MVP v1: Core Tools (T57)
- **Features**: Interactive Code Playground, VS Code Extension integration, Call Path Explorer, MCP Debugging Tools, and `summarize_architecture` MCP Tool.
- **Seeding**: Basic user accounts and one public repository graph.
- **Verification**: Ensure the Playwright test can open the call path explorer on a seeded graph, and the Rust CLI correctly generates the graph in a temporary workspace.

### MVP v2: Team Features (T58)
- **Features**: Architecture Drift Alerts, "Onboard Me" walkthroughs.
- **Seeding**: Multi-user team workspace, simulated Git history, and drift alert rules.
- **Verification**: Validate team RBAC (Role-Based Access Control) on graph visibility, and trigger an architecture drift alert via the CLI.

### MVP v3: Enterprise Features (T59)
- **Features**: OpenTelemetry Trace Overlay, Multi-Repo Cross-Service Graph, SBOM Export.
- **Seeding**: Multi-repo enterprise topology, mock OTEL traces, and SBOM metadata.
- **Verification**: Validate cross-repo graph rendering and SBOM export capabilities via the CLI.
