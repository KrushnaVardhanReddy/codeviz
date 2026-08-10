---
title: Interactive Call Path Explorer
tags: [ui, features, visualization, react-flow]
source_count: 1
---

# Interactive Call Path Explorer

The Interactive Call Path Explorer is a frontend feature built into the CodeViz Web UI (`codeviz-web`) that animates the execution flow of the codebase.

## Key Concepts
- **Client-Side Animation**: Users click a function node to reveal a "Trace Paths" button. Clicking this initiates a breadth-first search (BFS) animation over the `React Flow` graph state.
- **Pulsing Animation**: The traversal highlights nodes and edges sequentially with a 400ms delay between hops to visualize reachable paths.
- **Traversal State**: Includes controls to Play, Pause, Step Forward, Step Back, and Reset the animation, along with a step counter.
