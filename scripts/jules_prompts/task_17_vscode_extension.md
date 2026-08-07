TASK: T17 — VS Code Extension

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
1. Activate on any workspace containing `codeviz.toml`
2. Sidebar panel "CodeViz" showing current file's module graph
3. On file save: run `codeviz run --path . --diagram module` as child process
4. Render Mermaid output via Mermaid.js in a webview panel
5. Status bar item: "CodeViz: Ready" / "CodeViz: Parsing..." / "CodeViz: Error"
6. Optional: bundle WASM instead of requiring local binary (feature flag)

Files to Modify/Create:


Spec (READ ONLY — implement from it, never edit):
  docs/specs/TODO.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- Write comprehensive unit tests:

- No unwraps or panics in core parsing logic. Return Result.
- Ensure 'cargo clippy --all -- -D warnings' and 'cargo test --all' pass cleanly.
