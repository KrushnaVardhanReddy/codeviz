# Jules Task 10 — WASM Adapter (wasm-pack)

## Objective
Build the WASM adapter so CodeViz can run in the browser.

## Files to Modify/Create
- `codeviz-wasm/src/lib.rs`
- `codeviz-wasm/Cargo.toml`
- `codeviz-wasm/README.md` (usage instructions for JS consumers)
- `codeviz-wasm/index.html` (demo page)

## Requirements
1. Use `wasm-bindgen` to expose a JS-callable API:
```typescript
// Generated TypeScript signature
function parse(source: string, language: string, diagram_kind: string): string;
// Returns: Mermaid diagram string, or throws on error
```
2. `language` parameter: `"python"` | `"typescript"` | `"javascript"`
3. `diagram_kind` parameter: `"module"` | `"call"` | `"class"`
4. Bundle must compile with `wasm-pack build --target web`.
5. Bundle size must be < 3MB (enforce with a CI size check in `ci.yml`).
6. `index.html` demo: paste code in a textarea, select language, click "Generate" → renders Mermaid output.

## Unit Tests
- Add a `#[test]` in `lib.rs` testing the `parse` function with a simple Python snippet
- Assert the returned string starts with `graph TD` or `flowchart TD` or `classDiagram`
