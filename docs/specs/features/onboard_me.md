# Spec: "Onboard Me" — AI Architecture Walkthrough (Phase 20)

## Overview
A feature that generates a step-by-step, human-readable walkthrough of a codebase
for new engineers. Uses the `CodeGraph` to produce a guided narrative — no LLM
required. Think of it as auto-generated architecture documentation.

## CLI Usage
```bash
codeviz onboard --path . --output ARCHITECTURE.md
```

## Generated Document Structure
```markdown
# Architecture Walkthrough: {repo_name}

## 1. Entry Points
Start here: {entry_point_list with file paths and line numbers}

## 2. Core Modules
The most imported modules (by in-degree) are: ...

## 3. Dependency Flow
{Mermaid diagram of top-level module flow}

## 4. Key Abstractions
{Classes/interfaces with the most implementations/inheritors}

## 5. Health Summary
{table of files with health scores, flagging anything below 7.0}
```

## MCP Tool
Also expose as a new MCP tool:
```json
Tool: "onboard_codebase"
Input:  { "path": string }
Output: { "markdown": string }
```

## Files to Modify/Create
- `codeviz-core/src/onboard.rs` [NEW] — walkthrough generator
- `codeviz-cli/src/main.rs` — add `onboard` subcommand
- `codeviz-mcp/src/tools.rs` — add `onboard_codebase` tool

## Constraints
- No LLM or external API calls. Purely deterministic from the `CodeGraph`.
- `[TEAM]` tier in the SaaS.
- `cargo test --all` and `cargo clippy --all` must pass.
