# Spec: Multi-Repo Cross-Service Dependency Graph (Phase 21)

## Overview
Allow teams to define a set of repositories in `codeviz.toml` and visualize how
they depend on each other at the API boundary level. Targeted at microservice
architectures.

## Config Schema Extension
```toml
[workspace]
repos = [
  { name = "api-gateway", path = "../api-gateway" },
  { name = "auth-service", path = "../auth-service" },
  { name = "billing-service", path = "../billing-service" },
]
```

## Cross-Repo Edge Detection
Cross-repo edges are detected by matching:
1. HTTP client calls (e.g., `reqwest::get("http://auth-service/...")`)
2. Shared package imports (e.g., both repos import `@company/shared-types`)
3. gRPC proto imports

## Output
- A new `WorkspaceGraph` type that contains multiple `CodeGraph` instances.
- Renderable as a Mermaid `graph TD` with repo-level clustering.
- New CLI command: `codeviz workspace --path .`

## Files to Create/Modify
- `codeviz-core/src/workspace.rs` [NEW] — `WorkspaceGraph` type
- `codeviz-cli/src/main.rs` — add `workspace` subcommand
- `docs/specs/08_config_schema.md` — extend `[workspace]` section

## Constraints
- Single-repo mode must be completely unaffected.
- This is `[ENT]` tier — gate behind plan check in the web API.
- `cargo test --all` and `cargo clippy --all` must pass.
