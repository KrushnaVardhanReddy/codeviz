# Spec: WASM API

## Purpose
The WASM module exposes CodeViz's parsing engine to browser environments
with a zero-upload guarantee — no source code leaves the user's machine.

---

## Exported JS API (wasm-bindgen)
```typescript
/**
 * Parse source code and return a Mermaid diagram string.
 * @param source     - Full text content of the source file
 * @param language   - "python" | "typescript" | "javascript" | "go" | "rust" | "java"
 * @param diagram_kind - "module" | "call" | "class"
 * @returns Mermaid diagram string
 * @throws string error message on parse failure
 */
export function parse(source: string, language: string, diagram_kind: string): string;

/**
 * Parse and return the full CodeGraph as a JSON string.
 * @param source   - Full text content
 * @param language - Language identifier
 * @returns JSON-serialized CodeGraph
 */
export function parse_to_json(source: string, language: string): string;

/**
 * Return a list of supported language identifiers.
 */
export function supported_languages(): string[];  // e.g. ["python", "typescript"]
```

---

## Bundle Constraints
| Constraint | Limit |
|---|---|
| Total WASM bundle size | < 3 MB |
| Time to first parse (after init) | < 500ms |
| Supported environments | Chrome 90+, Firefox 88+, Safari 15+ |

---

## Initialization
The WASM module uses async initialization (wasm-bindgen default):
```javascript
import init, { parse } from './codeviz_wasm.js';
await init();
const diagram = parse(mySource, 'python', 'module');
```

---

## Error Handling
- `parse()` throws a `string` (not an Error object) on failure.
- Callers must wrap in `try/catch`.
- Invalid `language` → throws `"Unsupported language: <name>"`.
- Invalid `diagram_kind` → throws `"Unknown diagram kind: <name>"`.

---

## Acceptance Criteria
- `wasm-pack build --target web` succeeds without warnings.
- Bundle size verified by CI (fail if > 3MB).
- `parse("import os", "python", "module")` returns a string starting with `graph TD`.
- `parse("", "python", "module")` returns a valid empty graph (does not throw).
- `parse("x", "cobol", "module")` throws `"Unsupported language: cobol"`.
