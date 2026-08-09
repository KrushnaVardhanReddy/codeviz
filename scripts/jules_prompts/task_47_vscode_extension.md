TASK: T47 — VS Code Extension (codeviz-vscode)

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Build a VS Code extension that renders the CodeViz architecture graph
directly in the editor sidebar, updating on every file save.

Files to Create:
- `codeviz-vscode/package.json`
- `codeviz-vscode/src/extension.ts`
- `codeviz-vscode/src/graphPanel.ts`  (Webview + Mermaid renderer)
- `codeviz-vscode/src/statusBar.ts`
- `codeviz-vscode/README.md`

Spec (READ ONLY):
  docs/specs/features/vscode_extension.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- Activate only on workspaces with a `codeviz.toml`.
- Auto-detect local `codeviz` binary; fall back to WASM if not found.
- Status bar must show: Ready / Parsing... / Error states.
- `npm run compile` and `npm test` must pass.
- Do NOT modify any Rust crates.
