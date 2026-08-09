# CodeViz Task Tracker

This document tracks the progress of all CodeViz development phases and tasks.
Tasks are executed concurrently across different AI agents (Jules for backend/parsers, Stitch for UI, Antigravity for logic integration).

> [!NOTE]
> `[x]` = Done | `[/]` = In Progress | `[ ]` = Not Started

## Phase 0-5: Foundation & Core Parsers
- `[x]` **Batch 1 (Phase 0)**: Foundation (Tasks 1-5) - Core AST and Graph IR.
- `[x]` **Batch 2 (Phase 1)**: Python Parser + CLI + CI (Tasks 6-8)
- `[x]` **Batch 3 (Phase 2)**: TypeScript + WASM (Tasks 9-10)
- `[x]` **Batch 4 (Phase 3)**: MCP Server + Tests (Tasks 11-12)
- `[x]` **Batch 5 (Phase 4 & 5)**: Go + Rust Parsers (Tasks 13-14)

## Phase 6-10: Config & Tooling
- `[/]` **Batch 6 (Phase 6)**: Java + Kotlin Parsers (Tasks 15, 25) — *Running in Jules*
- `[/]` **Batch 7 (Phase 7)**: Config + Universal Parser (Tasks 16, 26, 40) — *Running in Jules*
- `[ ]` **Batch 8 (Phase 8)**: Critical Features (Tasks 18, 19, 21)
- `[ ]` **Batch 9 (Phase 9)**: Developer UX (Tasks 20, 22, 27)
- `[ ]` **Batch 10 (Phase 10)**: Distribution (GitHub Action + npm WASM) (Tasks 23, 24)

## Phase 11-12: Web UI & CFG
- `[/]` **Batch 11 (Phase 11)**: Web UI Scaffolding & Graph (Tasks 28, 29) — *Running in Stitch*
- `[ ]` **Batch 12 (Phase 12)**: Control Flow Graph & UI Side Panel (Tasks 30-32)

## Phase 13-16: Enterprise & Auth
- `[ ]` **Batch 13 (Phase 13)**: Auth (GitHub + Google OAuth) (Task 33)
- `[ ]` **Batch 14 (Phase 14)**: Teams & Workspaces (Task 34)
- `[ ]` **Batch 15 (Phase 15)**: Enterprise SSO & Audit Logs (Task 35)
- `[ ]` **Batch 16 (Phase 16)**: Enterprise Insights (Tasks 36-39)

## Phase 18: Advanced Analysis
- `[ ]` **Batch 18 (Phase 18)**: Advanced Analysis (Tasks 41-45)
  - Circular Dependencies, Unused Modules, PageRank, Health Score, Code Coverage.
