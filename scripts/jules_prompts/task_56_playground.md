TASK: T56 — Interactive Code Playground (Web UI)

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Create a public `/playground` route where users can paste code and instantly
see the CodeGraph rendered via the WASM module, entirely client-side.

Files to Create/Modify:
- `codeviz-web/app/playground/page.tsx` [NEW]
- `codeviz-web/components/PlaygroundEditor.tsx` [NEW]
- `codeviz-web/components/PlaygroundLayout.tsx` [NEW]

Spec (READ ONLY):
  docs/specs/features/playground.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- Must run 100% client-side using `codeviz-wasm`. No backend parsing.
- The route `/playground` must be public (not protected by middleware).
- Implement a split-pane layout: Editor on left, Graph on right.
- Debounce parser execution by 500ms to avoid freezing the UI.
- Use Monaco Editor (or CodeMirror) for the code input.
- Add at least 2 dropdown examples (e.g., Python, TS) to pre-fill the editor.
- Ensure `npm run build` passes cleanly.
