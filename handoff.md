# CodeViz Project Handoff

Welcome to the CodeViz project! This document provides the critical context and rules you need to seamlessly resume orchestration and development.

## 1. Project Overview
CodeViz is a full-stack code visualization engine that parses repositories to generate Mermaid, DOT, and JSON graph architectures. 
- **Core Backend**: Rust workspace (`codeviz-cli`, `codeviz-core`, `codeviz-mcp`, and language-specific parsers).
- **Frontend**: Next.js + React Flow (`codeviz-web/`).
- **Development Philosophy**: We strictly follow a **Spec-First Approach**.

## 2. The Spec-First Approach
This is the golden rule of the CodeViz repository:
- **Never write code first.** Before executing any new architectural feature (or orchestrating Jules/Stitch to do it), you MUST draft a detailed markdown specification in the `docs/specs/` directory.
- After drafting the spec, you MUST ingest it into the LLM Wiki (`.agents/wiki/`) by creating a concept page, updating `wiki/index.md`, and logging it in `wiki/log.md`.
- Only *after* the spec is approved and ingested should you update the Jules/Stitch prompts to point to the new spec file and trigger the execution.

## 3. Your Role (Antigravity)
**You are strictly the Orchestrator and Debugger.**
- **NO DIRECT FEATURE CODING**: Do not write or execute feature code yourself. 
- **Orchestrate**: Delegate tasks using `python3 scripts/jules_submit.py --task <N>` for backend/infrastructure, and `python3 scripts/stitch_submit.py` for Web UI generation.
- **Debug**: Your job is to review PRs, fix build failures, resolve merge conflicts, and manage the architecture specifications.

## 3. Important Locations & Wiki
- **Tracker**: `docs/tracker.md` is the absolute source of truth for the project timeline and task batches. Check this first.
- **LLM Wiki**: `.agents/wiki/` is our persistent, compounding knowledge base. 
  - ALWAYS follow the **Ingest Protocol** (defined in `.agents/AGENTS.md`) when creating new specs: Read source $\rightarrow$ Create concept page in `wiki/pages/` $\rightarrow$ Update `wiki/index.md` $\rightarrow$ Log in `wiki/log.md`.
- **Jules Prompts**: `scripts/jules_prompts/` contains the task directives sent to Jules.

## 4. Current State (As of Last Session)
We have successfully validated, resolved conflicts, and merged Batch 9 (T20, T22, T27) and Task 33C (SurrealDB Migration) into the `main` branch. 

We are now currently awaiting asynchronous tasks running in Jules' cloud sandboxes for the next phase of development:
- ⏳ **Batch 10**: Distribution (GitHub Action + npm WASM)
- ⏳ **Batch 12**: Control Flow Graph (IR, Parsers, Web UI)
- ⏳ **Batch 20**: MVP v1 Core Tools (VS Code Extension, MCP Summarizer, Call Path Explorer)

**Next Steps upon resuming:**
1. Wait for and validate Jules' PRs for Batches 10, 12, and 20.
2. Run automated tests (`npm run test:e2e` and `cargo test`) on each branch.
3. Resolve any minor merge conflicts (e.g., in `DetailPanel.tsx` between Batch 12 and Batch 20).
4. Merge them into `main` one by one.
5. Trigger the next batch of tasks (e.g., Batch 14 for Teams).
