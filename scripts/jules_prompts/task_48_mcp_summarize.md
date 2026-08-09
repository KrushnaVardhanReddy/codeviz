TASK: T48 — `summarize_architecture` MCP Tool

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Add a new `summarize_architecture` MCP tool that returns a human-readable
narrative summary of a codebase from its CodeGraph. No LLM required —
purely deterministic from graph data.

Files to Modify:
- `codeviz-core/src/lib.rs`   (add `summarize()` method to `CodeGraph`)
- `codeviz-mcp/src/tools.rs` (add `summarize_architecture` handler)

Spec (READ ONLY):
  docs/specs/features/mcp_summarize.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- Summary generation must be 100% deterministic. Same graph → same string.
- No external API calls. No LLM dependencies.
- Do NOT change existing MCP tool signatures.
- Ensure `cargo test --all` and `cargo clippy --all -- -D warnings` pass.
