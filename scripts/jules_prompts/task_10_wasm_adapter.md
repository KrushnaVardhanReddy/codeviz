TASK: T10 — WASM Adapter (wasm-pack)

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Build the WASM adapter so CodeViz can run in the browser.
1. Use `wasm-bindgen` to expose a JS-callable API:
   function parse(source: string, language: string, diagram_kind: string): string;
2. `language` parameter: `"python"` | `"typescript"` | `"javascript"`
3. `diagram_kind` parameter: `"module"` | `"call"` | `"class"`
4. Bundle must compile with `wasm-pack build --target web`.
5. Bundle size must be < 3MB (enforce with a CI size check in `ci.yml`).
6. `index.html` demo: paste code in a textarea, select language, click "Generate" → renders Mermaid output.

Files to Modify/Create:
- `codeviz-wasm/src/lib.rs`
- `codeviz-wasm/Cargo.toml`
- `codeviz-wasm/README.md` (usage instructions for JS consumers)
- `codeviz-wasm/index.html` (demo page)

Spec (READ ONLY — implement from it, never edit):
  docs/specs/TODO.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- Write comprehensive unit tests:
  - Add a `#[test]` in `lib.rs` testing the `parse` function with a simple Python snippet
  - Assert the returned string starts with `graph TD` or `flowchart TD` or `classDiagram`
- No unwraps or panics in core logic. Return Result.
- Ensure 'cargo clippy --all -- -D warnings' and 'cargo test --all' pass cleanly.

═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- The `< 3MB` check is ALREADY implemented in `.github/workflows/ci.yml` under the `wasm-size` job. You DO NOT need to touch `ci.yml`.
- Task 09 (the TypeScript parser) is being developed concurrently and does not exist in `main` yet. To get `codeviz-wasm` to compile when supporting `"typescript"`, please create a basic stub `codeviz-typescript` crate locally in your branch. We will resolve the conflicts when merging!
- DO NOT use the unstable `let_chains` feature! You must use stable Rust 2021 edition.
