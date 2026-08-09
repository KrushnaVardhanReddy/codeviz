# Spec: Interactive Code Playground (MVP v1)

## Overview
A public-facing "Playground" or "Sandbox" on the CodeViz website (e.g., `/playground`) where users can paste arbitrary code snippets in any supported language and instantly see the generated `CodeGraph` (rendered as a flowchart). 

This is a critical **Top of Funnel (ToFu)** feature for user acquisition. It allows developers to experience the "aha moment" without installing the CLI or signing up.

## How It Works
1. The page has a split-pane layout:
   - **Left Pane:** A Monaco Editor (or CodeMirror) instance where users type/paste code.
   - **Right Pane:** The React Flow canvas rendering the CodeViz graph.
2. **Language Selector:** A dropdown above the editor to select the language (Rust, Python, TS, Go, etc.).
3. **Execution:** On every keystroke (debounced by 500ms), the code is passed to the **CodeViz WASM module**.
4. The WASM module parses the code locally in the browser (zero latency, zero server costs) and returns the JSON CodeGraph.
5. The React Flow pane updates instantly to reflect the new architecture.

## Requirements
- Use the WASM adapter built in Task 10 (`codeviz-wasm`).
- Must run 100% client-side. No backend API calls for parsing (this keeps server costs at $0 even if it goes viral on Hacker News).
- Provide 3-4 "Examples" (e.g., a simple Express.js server, a Rust CRUD app) that users can click to pre-fill the editor.
- Add an "Export" button to download the graph as SVG or PNG.

## Files to Create/Modify
- `codeviz-web/app/playground/page.tsx` [NEW]
- `codeviz-web/components/PlaygroundEditor.tsx` [NEW]
- `codeviz-web/components/PlaygroundLayout.tsx` [NEW]

## Constraints
- This is a `[OSS]` public route, completely unauthenticated.
- Ensure the WASM bundle is loaded asynchronously so it doesn't block the initial page render.
- Handle syntax errors gracefully (show a small error badge, don't crash the UI).
