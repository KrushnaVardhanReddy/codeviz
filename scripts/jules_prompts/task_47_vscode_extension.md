TASK: T47 — VS Code Extension (codeviz-vscode)

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Build a VS Code extension that renders the CodeViz architecture graph
directly in the editor sidebar, updating on every file save.

Files to Create/Update:
- `codeviz-vscode/package.json` (Requires "vscode" engine ^1.85.0 and appropriate dependencies)
- `codeviz-vscode/tsconfig.json`
- `codeviz-vscode/src/extension.ts`
- `codeviz-vscode/src/graphPanel.ts` (Implement vscode.WebviewViewProvider)
- `codeviz-vscode/src/statusBar.ts`
- `codeviz-vscode/README.md`

Spec (READ ONLY):
  docs/specs/features/vscode_extension.md

═══════════════════════════════════════════════════════════════
IMPLEMENTATION DETAILS & CONTEXT
═══════════════════════════════════════════════════════════════
- **Webview Provider:** Use `vscode.window.registerWebviewViewProvider` to create a sidebar panel for the graph. The HTML must include a `<div class="mermaid">` and load Mermaid.js via CDN (e.g., cdnjs).
- **Execution:** When the active text editor changes or on document save, run `codeviz run --path . --diagram module`. If successful, send the Mermaid string via `webview.postMessage` to the panel to render.
- **WASM Fallback:** If `codeviz.useWasm` is true, import the `codeviz` NPM package (from the `npm/` directory) and use `parse_and_build_graph` + `render_graph`.
- **Status Bar:** Create a status bar item. Show `$(sync~spin) CodeViz: Parsing...` while parsing and `$(check) CodeViz: Ready` when done.
- **Commands:** Register `codeviz.showGraph`, `codeviz.refreshGraph`, and `codeviz.openWebUi`.

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- Activate only on workspaces with a `codeviz.toml`.
- Auto-detect local `codeviz` binary; fall back to WASM if not found.
- Status bar must show: Ready / Parsing... / Error states.
- `npm run compile` and `npm test` must pass.
- Do NOT modify any Rust crates.
