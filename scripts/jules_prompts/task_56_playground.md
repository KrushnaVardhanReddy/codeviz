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

═══════════════════════════════════════════════════════════════
CRITICAL ARCHITECTURAL REQUIREMENTS (DO NOT IGNORE)
═══════════════════════════════════════════════════════════════
1. **NO STRING MATCHING**: You must NOT use string matching or regex for parsing. The Playground must generate the exact same AST/Graph as the CLI using Tree-sitter.
2. **WASM Constraints**: If compiling the Rust tree-sitter C-bindings to `wasm32-unknown-unknown` fails due to standard library constraints, **do not fallback to regex in Rust**. Instead:
   - Use the official `web-tree-sitter` NPM package directly in the Next.js frontend to parse the code into an AST.
   - Pass that JSON AST *into* a simplified `codeviz-wasm` Rust function that only constructs the `CodeGraph` and returns it.
3. **Clippy Bypasses**: Since `let_chains` is unstable in Rust, you are explicitly authorized to use `#[allow(clippy::collapsible_if)]` for nested `if let` blocks if Clippy complains.
