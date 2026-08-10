TASK: T50 — Multi-Repo Cross-Service Dependency Graph

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Enable visualization of dependencies across multiple repositories (microservices).
This involves expanding the `codeviz.toml` schema to accept multiple repo paths,
building a higher-level `WorkspaceGraph`, and detecting cross-repo edges like
HTTP client calls or shared package imports.

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/multi_repo_graph.md

═══════════════════════════════════════════════════════════════
BACKGROUND — WHAT ALREADY EXISTS
═══════════════════════════════════════════════════════════════

- The single-repo `CodeGraph` exists in `codeviz-core/src/graph.rs`.
- Configuration parsing exists for `codeviz.toml`.
- Parsers generate standard `Node` and `Edge` structs.

═══════════════════════════════════════════════════════════════
DELIVERABLES
═══════════════════════════════════════════════════════════════

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. MODIFY: Config parsing to support workspaces
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Extend `codeviz.toml` parsing to support a `[workspace]` array:
```toml
[workspace]
repos = [
  { name = "api-gateway", path = "../api-gateway" },
  { name = "auth-service", path = "../auth-service" }
]
```

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
2. CREATE: codeviz-core/src/workspace.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Create a new `WorkspaceGraph` struct that contains multiple `CodeGraph` instances.
Implement cross-repo edge detection. For example:
- Detect HTTP client calls (e.g. `http://auth-service/`).
- Detect shared package imports.

Register this module in `codeviz-core/src/lib.rs`.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
3. MODIFY: codeviz-core/src/render/
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Extend the rendering logic to support `WorkspaceGraph`.
In Mermaid output, use `subgraph` clustering to group nodes by their parent repository.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
4. MODIFY: codeviz-cli/src/main.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Add a new `workspace` subcommand:
```bash
codeviz workspace --path <dir-with-workspace-toml>
```
This command should read the workspace config, parse all repos, resolve cross-repo edges, and output the clustered graph.

═══════════════════════════════════════════════════════════════
CRITICAL IMPLEMENTATION RULES
═══════════════════════════════════════════════════════════════
1. Single-repo mode must remain completely unaffected. Do not break existing CLI commands.
2. This is an `[ENT]` tier feature.
3. Ensure `cargo test --all` and `cargo clippy --all -- -D warnings` pass.
4. Add unit tests in `workspace.rs` for cross-repo edge resolution.

Commit: "jules: T50 — Multi-Repo Cross-Service Dependency Graph"
Target branch: feat-t50-multi-repo
