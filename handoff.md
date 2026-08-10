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
We have successfully validated, resolved conflicts, and merged the following into the `main` branch:
- **Batch 9** (T20, T22, T27) and **Task 33C** (SurrealDB Migration).
- **Batch 10** (T23: GitHub Action, T24: npm WASM).
- **Batch 12** (T30: CFG IR, T31: CFG Parsers, T32: CFG UI).
- **Batch 20** (T47: VS Code Extension, T48: MCP Summarize, T53: Call Path Explorer).

We are currently awaiting asynchronous tasks running in Jules' cloud sandboxes for the next phase of development:
- ⏳ **Batch 21**: MVP v1 Full E2E (T57: E2E Test Suite, T60: Path Normalization, T61: VS Code E2E)

**Next Steps upon resuming:**
1. Wait for and validate Jules' PRs for Batch 21.
2. Resolve any minor merge conflicts (e.g. in `.github/workflows/ci.yml` between T57 and T60).
3. Merge them into `main`.
4. Run final automated tests (`npm run test:e2e` and `cargo test`) to confirm MVP v1 completion.
5. Trigger the next batch of tasks for MVP v2.
