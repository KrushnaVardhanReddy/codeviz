---
title: MCP Architecture Summarizer
tags: [mcp, tools, analysis, llm]
source_count: 1
---

# MCP Architecture Summarizer

The `summarize_architecture` MCP tool provides a human-readable narrative summary of a repository's architecture. It is designed to give LLMs an immediate high-level mental model of the codebase without requiring exhaustive graph traversal.

## Key Concepts
- **Deterministic Output**: The summary is generated deterministically directly from the `CodeGraph` structure without calling any external LLM APIs.
- **Data Points**: Includes counts (files, nodes, edges, circular dependencies), top entry points, and most-imported modules.
- **Integration**: Plugs into the `codeviz-mcp` crate as a standard JSON-RPC tool endpoint.
