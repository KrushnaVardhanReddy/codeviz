---
title: "LLM Wiki Pattern"
tags: ["architecture", "agents", "knowledge-management"]
source_count: 1
---

# LLM Wiki Pattern

The LLM Wiki pattern is a methodology for maintaining a persistent, compounding knowledge base where an AI Agent acts as the sole maintainer of the structured content.

## Core Concepts
Instead of using purely extractive Retrieval-Augmented Generation (RAG) where an LLM must rediscover connections every time a query is made, this pattern introduces a persistent intermediate layer: **the wiki**.

- **Raw Sources**: The immutable input documents (articles, specs, transcripts).
- **The Wiki**: A structured, interlinked collection of markdown files. The agent reads raw sources and synthesizes the knowledge into concept pages, updating them dynamically.
- **The Schema (`AGENTS.md`)**: The set of instructions telling the agent how to maintain the wiki (Ingest, Query, Lint protocols).

## Operations
1. **Ingest**: The agent reads a new source, creates or updates relevant concept pages, updates the `index.md`, and logs the action in `log.md`.
2. **Query**: The agent searches the `index.md`, reads the relevant concept pages, and synthesizes an answer. If the answer produces a novel insight, it is saved back into the wiki.
3. **Lint**: The agent performs periodic maintenance to fix orphaned pages, resolve contradictions, and clean up the graph.

## Implementation in CodeViz
CodeViz utilizes this exact pattern within the `.agents/wiki/` directory. All agents (Jules, Stitch, Antigravity) are bound by the `.agents/AGENTS.md` schema to autonomously update this knowledge base whenever they ingest new architectural specs.

---
**Sources**:
- `raw/llm-wiki.md` (Karpathy's LLM Wiki Gist)
