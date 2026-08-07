TASK: T09 — TypeScript/JavaScript Parser

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement the TypeScript/JavaScript language parser using Tree-sitter.
Use `tree-sitter-typescript` (covers TS, TSX, and JS).
Extract into `CodeGraph`:
1. `import { X } from 'y'` (ESM) → `Edge { kind: Imports }`
2. `const x = require('y')` (CJS) → `Edge { kind: Imports }`
3. `class Foo extends Bar` → `Node { Class }` + `Edge { Inherits }`
4. `interface IFoo` → `Node { Interface }`
5. `function foo()` / arrow functions → `Node { Function }`
6. `export default` / named exports → mark node with `is_public: true` metadata

Handle gracefully (skip, don't panic):
- Dynamic `import('...')` calls
- Barrel files (`index.ts` re-exports)
- `.tsx` / `.jsx` JSX syntax

Files to Modify/Create:
- `codeviz-typescript/Cargo.toml`
- `codeviz-typescript/src/lib.rs`
- `codeviz-typescript/src/parser.rs`

Spec (READ ONLY — implement from it, never edit):
  docs/specs/parsers/typescript.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- Write comprehensive unit tests:
  - Parse ESM imports and assert correct `Imports` edges
  - Parse CJS `require()` and assert correct `Imports` edges
  - Parse class inheritance and assert `Inherits` edge
  - Parse `async function` and assert `is_async: true` on the node
- No unwraps or panics in core parsing logic. Return Result.
- Ensure 'cargo clippy --all -- -D warnings' and 'cargo test --all' pass cleanly.

═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- DO NOT use the unstable `let_chains` feature! You must use stable Rust 2021 edition. If `cargo clippy --fix` introduces `let_chains`, you must manually unroll them into nested `if let` blocks.
- The `tree-sitter-typescript` crate exports both `language_typescript()` and `language_tsx()`. Make sure your code conditionally switches to the `tsx` grammar specifically when parsing `.tsx` and `.jsx` files, otherwise it will fail to parse JSX syntax!
