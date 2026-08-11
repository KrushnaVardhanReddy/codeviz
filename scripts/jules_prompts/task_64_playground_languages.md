TASK: T64 — Playground Additional Languages

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Update the Interactive Code Playground (http://localhost:3003/playground) to natively
support Go, Rust, Java, and Kotlin in the browser using their respective WebAssembly
(WASM) tree-sitter grammars. Update the WASM AST extraction logic to accurately map
the AST nodes of these new languages into CodeGraph objects.

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/playground_languages.md

═══════════════════════════════════════════════════════════════
BACKGROUND — WHAT ALREADY EXISTS (READ ONLY — do not modify unless specified)
═══════════════════════════════════════════════════════════════

THE PROBLEM:
  The CodeViz web playground (`codeviz-web/components/PlaygroundLayout.tsx`)
  currently only offers `python` and `typescript` as language choices. The WASM
  extraction logic (`codeviz-wasm/src/lib.rs`) assumes generic tree-sitter node
  types (e.g. `class_definition`, `function_definition`) which map cleanly to Python
  and JS, but fail to extract components for Rust (`struct_item`, `impl_item`), Go
  (`type_declaration`), or Java/Kotlin.

WHAT'S ALREADY IN PLACE:
  - All necessary WebAssembly grammar files for Go, Rust, Java, and Kotlin are
    already present in `codeviz-web/public/tree-sitter-wasms/`.
  - The dynamic loader for these `.wasm` files is already working for TS/Python.
  - The CodeViz core IR (`codeviz-core/src/ir.rs`) supports generic Node types
    like `NodeKind::Class`, `NodeKind::Function` which apply uniformly.

═══════════════════════════════════════════════════════════════
DELIVERABLES
═══════════════════════════════════════════════════════════════

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. MODIFY: codeviz-web/components/PlaygroundLayout.tsx
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

- Update the Language dropdown to include `go`, `rust`, `java`, and `kotlin`.
- Add default code snippets for each new language in the `EXAMPLES` constant map.
  (Ensure they define a simple class/struct and a method, then instantiate it).
- Ensure the WASM file name loading logic correctly maps the new languages.
  (e.g., `rust` maps to `tree-sitter-rust.wasm`, `go` maps to `tree-sitter-go.wasm`).

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
2. MODIFY: codeviz-wasm/src/lib.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

- Update the `extract_from_ast` function to handle language-specific AST nodes.
- **Rust**: Add support for `struct_item` (maps to Class), `impl_item`, `function_item` (maps to Function).
- **Go**: Add support for `type_declaration` (maps to Class for structs/interfaces), `method_declaration`, `function_declaration`.
- **Java/Kotlin**: Add support for `class_declaration`, `method_declaration`, `object_declaration` (Kotlin).
- Make sure `Contains`, `Calls`, `Inherits`, and `Instantiates` edges continue
  to be mapped appropriately based on the AST structure of these new languages.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
3. VERIFY: Compile and Test
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

- Run `cd codeviz-wasm && wasm-pack build --target web` to ensure WASM builds without panics.
- Run `cd codeviz-web && npm run build` to ensure the frontend compiles.

═══════════════════════════════════════════════════════════════
CRITICAL IMPLEMENTATION RULES
═══════════════════════════════════════════════════════════════

1. Do NOT add new Rust crates or npm dependencies.
2. Handle node parsing defensively to prevent WASM panics if a language's AST uses unexpected nodes.
3. Do NOT modify any core Rust logic outside of `codeviz-wasm/src/lib.rs` (e.g. do not touch `codeviz-core`).
4. Do NOT modify any spec files (`docs/specs/`).

═══════════════════════════════════════════════════════════════
FILES LIST
═══════════════════════════════════════════════════════════════

FILES TO MODIFY:
  codeviz-web/components/PlaygroundLayout.tsx
  codeviz-wasm/src/lib.rs

FILES NOT TO TOUCH (READ-ONLY):
  codeviz-core/                         (all core IR and parser code)
  docs/specs/                           (all spec files)

Commit: "jules: T64 — Support Go, Rust, Java, Kotlin in Web Playground"
Target branch: feat-t64-playground-languages
