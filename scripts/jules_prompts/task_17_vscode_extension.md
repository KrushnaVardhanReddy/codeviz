# Jules Task 17 — VS Code Extension

## Files to Create
- `vscode-extension/package.json`
- `vscode-extension/src/extension.ts`
- `vscode-extension/src/panel.ts`
- `vscode-extension/README.md`

## Requirements
1. Activate on any workspace containing `codeviz.toml`
2. Sidebar panel "CodeViz" showing current file's module graph
3. On file save: run `codeviz run --path . --diagram module` as child process
4. Render Mermaid output via Mermaid.js in a webview panel
5. Status bar item: "CodeViz: Ready" / "CodeViz: Parsing..." / "CodeViz: Error"
6. Optional: bundle WASM instead of requiring local binary (feature flag)

## Tests
Set up basic Mocha/Chai tests for extension activation and panel creation.
