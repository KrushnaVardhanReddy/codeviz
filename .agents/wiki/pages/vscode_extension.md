---
title: VS Code Extension
tags: [vscode, client, integration, ui]
source_count: 1
---

# VS Code Extension

CodeViz provides a native VS Code Extension (`codeviz.codeviz`) that brings the visualization engine directly into the editor.

## Key Concepts
- **Automatic Activation**: Activates automatically when a `codeviz.toml` file is present in the workspace root.
- **Sidebar Integration**: Provides a "CodeViz Explorer" sidebar panel containing a webview that renders the dependency graph (using Mermaid.js) for the currently focused file.
- **Real-time Updates**: Automatically re-runs `codeviz run --path . --diagram module` and refreshes the graph whenever a file is saved.
- **Status Bar**: A status bar item displays the current engine state (`CodeViz: Ready`, `Parsing...`, `Error`) and links to the output channel.
- **WASM Fallback**: Supports an experimental WASM mode via the `codeviz.useWasm` setting to parse code directly inside the extension without requiring the native CLI binary.
