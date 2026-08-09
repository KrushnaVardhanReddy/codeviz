TASK: T50 — Multi-Repo Cross-Service Dependency Graph

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Allow teams to define multiple repos in `codeviz.toml` and visualize
cross-service dependencies as a unified `WorkspaceGraph`.

Files to Create/Modify:
- `codeviz-core/src/workspace.rs` [NEW]
- `codeviz-cli/src/main.rs`  (add `workspace` subcommand)
- `docs/specs/08_config_schema.md`  (extend `[workspace]` section)

Spec (READ ONLY):
  docs/specs/features/multi_repo_graph.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- Single-repo mode must be completely unaffected.
- This is an [ENT] tier feature — add a plan-check stub (return error if plan != enterprise).
- Ensure `cargo test --all` and `cargo clippy --all -- -D warnings` pass.
