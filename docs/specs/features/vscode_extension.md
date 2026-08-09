# Spec: VS Code Extension (Phase 20)

## Overview
A VS Code extension that brings CodeViz directly into the editor. It activates
on any workspace containing a `codeviz.toml` and renders the current file's
module graph in a sidebar webview, updating on every save.

## Activation
- Activate when `codeviz.toml` is found in the workspace root.
- Extension ID: `codeviz.codeviz`

## Features

### Sidebar Panel ("CodeViz Explorer")
- Renders the dependency graph for the **currently focused file**.
- Uses Mermaid.js inside a VS Code Webview.
- Updates automatically on file save (runs `codeviz run --path . --diagram module`).

### Status Bar Item
- Shows `CodeViz: Ready` / `CodeViz: Parsing...` / `CodeViz: Error`.
- Clicking it opens the Output channel with the last error.

### Commands (Command Palette)
- `CodeViz: Show Graph` — focus the sidebar panel.
- `CodeViz: Refresh Graph` — force re-parse.
- `CodeViz: Open in Web UI` — open the workspace graph in the CodeViz web app.

### WASM Mode (optional, behind feature flag)
- If `codeviz.useWasm: true` in settings, use the bundled WASM adapter
  instead of requiring a local `codeviz` binary.

## Files to Create
- `codeviz-vscode/` — new package at workspace root
- `codeviz-vscode/package.json`
- `codeviz-vscode/src/extension.ts`
- `codeviz-vscode/src/graphPanel.ts` — Webview + Mermaid renderer
- `codeviz-vscode/src/statusBar.ts`
- `codeviz-vscode/README.md`

## Constraints
- Must not require the user to install anything beyond the extension itself
  (the binary path should be auto-detected or fall back to WASM).
- Works on VS Code 1.85+.
- `npm run compile` and `npm test` must pass.
