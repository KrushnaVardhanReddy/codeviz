TASK: T52 — "Onboard Me" AI Architecture Walkthrough

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Generate a step-by-step, human-readable markdown walkthrough of a codebase
for new engineers. Purely deterministic — no LLM or external API required.

Files to Create/Modify:
- `codeviz-core/src/onboard.rs` [NEW]
- `codeviz-cli/src/main.rs`  (add `onboard` subcommand)
- `codeviz-mcp/src/tools.rs` (add `onboard_codebase` MCP tool)

Spec (READ ONLY):
  docs/specs/features/onboard_me.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- No LLM or external API calls. Must be 100% deterministic.
- Output must be valid Markdown.
- Ensure `cargo test --all` and `cargo clippy --all -- -D warnings` pass.
