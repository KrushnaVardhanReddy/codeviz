# CodeViz Task Tracker

This document tracks the progress of all CodeViz development phases and tasks.
Tasks are executed concurrently across different AI agents (Jules for backend/parsers, Stitch for UI, Antigravity for logic integration).

> [!NOTE]
> `[x]` = Done | `[/]` = In Progress | `[ ]` = Not Started
>
> **Open-Core Model**: `[OSS]` = MIT Licensed (free forever) | `[TEAM]` = $12/seat/mo | `[BIZ]` = $30/seat/mo | `[ENT]` = Enterprise contract

## Phase 0-5: Foundation & Core Parsers
- `[x]` **Batch 1 (Phase 0)**: Foundation (Tasks 1-5) - Core AST and Graph IR.
- `[x]` **Batch 2 (Phase 1)**: Python Parser + CLI + CI (Tasks 6-8)
- `[x]` **Batch 3 (Phase 2)**: TypeScript + WASM (Tasks 9-10)
- `[x]` **Batch 4 (Phase 3)**: MCP Server + Tests (Tasks 11-12)
- `[x]` **Batch 5 (Phase 4 & 5)**: Go + Rust Parsers (Tasks 13-14)

## Phase 6-10: Config & Tooling
- `[x]` **Batch 6 (Phase 6)**: Java + Kotlin Parsers (Tasks 15, 25)
- `[x]` **Batch 7 (Phase 7)**: Config + Universal Parser (Tasks 16, 26, 40)
- `[ ]` **Batch 8 (Phase 8)**: Critical Features (Tasks 18, 19, 21)
- `[ ]` **Batch 9 (Phase 9)**: Developer UX (Tasks 20, 22, 27)
- `[ ]` **Batch 10 (Phase 10)**: Distribution (GitHub Action + npm WASM) (Tasks 23, 24)

## Phase 11-12: Web UI & CFG
- `[x]` **Batch 11 (Phase 11)**: Web UI Scaffolding & Graph (Tasks 28, 29)
- `[/]` **Batch 12 (Phase 12)**: Control Flow Graph & UI Side Panel (Tasks 30-32)

## Phase 13-17: Enterprise, Auth & E2E
- `[x]` **Batch 13 (Phase 13)**: `[OSS]` Auth Core (NextAuth + GitHub/Google OAuth) (Task 33A)
- `[/]` **Batch 13.5 (Phase 13)**: `[OSS]` Auth DB Adapter & Playwright E2E (Task 33B)
- `[ ]` **Batch 14 (Phase 14)**: `[OSS]` Basic Teams & Workspaces (≤5 members) (Task 34)
- `[ ]` **Batch 15 (Phase 15)**: `[ENT]` Enterprise SSO (SAML) & Audit Logs (Task 35)
- `[ ]` **Batch 16 (Phase 16)**: `[BIZ]` Enterprise Insights — Heatmap, Blast Radius, Git History (Tasks 36-39)
- `[ ]` **Batch 17 (Phase 17)**: `[OSS]` E2E Validation Suite (CLI, MCP, Playwright) (Task 17)

## Phase 18: Advanced Analysis
- `[ ]` **Batch 18 (Phase 18)**: `[BIZ]` Advanced Analysis (Tasks 41-45)
  - Circular Deps `[OSS]`, Unused Modules `[OSS]`, PageRank `[BIZ]`, Health Score `[BIZ]`, Code Coverage `[TEAM]`.

## Phase 19: Semantic Search
- `[ ]` **Batch 19 (Phase 19)**: `[TEAM]` Semantic Code Search with LanceDB (Task 46)
  - Natural-language search over the CodeGraph using LanceDB + OpenAI embeddings.
  - Requires: Task 33 (Auth) + Task 34 (Teams) + Phase 18 complete.

---

## 🗺️ Product Roadmap — MVP Versions

### MVP v1 — "Make It Viral" (Core OSS — Adoption Flywheel)
> Goal: Get developers to install CodeViz and tell their friends.

- `[ ]` **T47** `[OSS]` VS Code Extension — sidebar graph panel, status bar, auto-refresh on save
- `[ ]` **T48** `[OSS]` `summarize_architecture` MCP Tool — instant codebase overview for AI agents
- `[ ]` **T55** `[OSS]` MCP Debugging Tools — `trace_call_path`, `get_callers_recursive`, `get_blast_radius`
- `[ ]` **T53** `[OSS]` Interactive Call Path Explorer — animated BFS graph traversal in Web UI

### MVP v2 — "Make It Sticky" (Team Features — Retention & Collaboration)
> Goal: Make teams embed CodeViz in their daily workflow.

- `[ ]` **T49** `[TEAM]` Architecture Drift Alerts — PR comments + Slack when arch regresses
- `[ ]` **T52** `[TEAM]` "Onboard Me" — auto-generated architecture walkthrough doc + MCP tool

### MVP v3 — "Make It Pay" (Enterprise Features — Revenue)
> Goal: Close enterprise contracts.

- `[ ]` **T54** `[BIZ]`  OpenTelemetry Trace Overlay — import OTEL trace, see live execution path on graph
- `[ ]` **T50** `[ENT]`  Multi-Repo Cross-Service Graph — visualize microservice dependencies
- `[ ]` **T51** `[ENT]`  SBOM Export (CycloneDX / SPDX) — compliance requirement for regulated industries

