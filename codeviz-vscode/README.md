# CodeViz for VS Code

CodeViz Explorer brings architecture and dependency graphs directly into your editor.
It activates on any workspace containing a `codeviz.toml` and renders the current file's module graph in a sidebar webview, updating on every save.

## Features

- **Sidebar Panel:** Renders the dependency graph for the currently focused file using Mermaid.js.
- **Status Bar:** Shows `CodeViz: Ready`, `Parsing...`, or `Error`. Clicking it opens the Output channel.
- **Commands:**
  - `CodeViz: Show Graph`
  - `CodeViz: Refresh Graph`
  - `CodeViz: Open in Web UI`
- **WASM Mode:** If `codeviz.useWasm` is true in settings, it uses the bundled WASM adapter.
