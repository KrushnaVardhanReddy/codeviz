TASK: T52 — "Onboard Me" AI Architecture Walkthrough

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement an "Onboard Me" feature that generates a deterministic, step-by-step,
human-readable architectural walkthrough document for new engineers, purely by
analyzing the `CodeGraph`. Expose this via a new CLI subcommand and a new MCP tool.

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/onboard_me.md

═══════════════════════════════════════════════════════════════
BACKGROUND — WHAT ALREADY EXISTS
═══════════════════════════════════════════════════════════════

- The `CodeGraph` structure is available in `codeviz-core/src/graph.rs`.
- The CLI entry point is `codeviz-cli/src/main.rs`.
- The MCP tools registry is in `codeviz-mcp/src/tools.rs`.

═══════════════════════════════════════════════════════════════
DELIVERABLES
═══════════════════════════════════════════════════════════════

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. CREATE: codeviz-core/src/onboard.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Implement the `generate_walkthrough(graph: &CodeGraph) -> String` function.
It must purely use the `CodeGraph` (no LLMs) to generate a Markdown document.

The document MUST contain:
1. Entry Points (nodes with no incoming Calls edges)
2. Core Modules (nodes with the highest in-degree)
3. Dependency Flow (A mermaid graph of the top-level module flow)
4. Key Abstractions (Interfaces/Traits with the most implementations)
5. Health Summary (Table of files with health scores < 7.0)

Register this module in `codeviz-core/src/lib.rs`.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
2. MODIFY: codeviz-cli/src/main.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Add a new `onboard` subcommand:
```bash
codeviz onboard --path <dir> [--output ARCHITECTURE.md]
```
When executed, it should parse the given path, call `generate_walkthrough`,
and print the result to stdout (or save it to the specified file).

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
3. MODIFY: codeviz-mcp/src/tools.rs & server.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Expose this feature as a new MCP tool:
- Tool Name: `onboard_codebase`
- Inputs: `{ "path": string }`
- Output: `{ "markdown": string }`

Ensure it is registered in `tools/list` and handled in `tools/call`.

═══════════════════════════════════════════════════════════════
CRITICAL IMPLEMENTATION RULES
═══════════════════════════════════════════════════════════════
1. Do NOT use any LLM APIs or external services. The generation must be 100%
   deterministic and based entirely on graph analytics.
2. Ensure `cargo test --all` and `cargo clippy --all -- -D warnings` pass.
3. Write unit tests for the `generate_walkthrough` function to verify the output formatting.

Commit: "jules: T52 — Onboard Me architecture walkthrough generator"
Target branch: feat-t52-onboard-me
