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

**Next Steps upon resuming:**
1. Check `docs/tracker.md` for the next batch of tasks.
2. Draft specs for the next tasks (e.g., Batch 10 for Distribution, Batch 14 for Teams & Workspaces, or T48 MCP Tool) and ingest them into the LLM Wiki.
3. Update Jules' prompts and trigger execution for the new tasks using `python3 scripts/jules_submit.py`.
