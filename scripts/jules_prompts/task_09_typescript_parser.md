# Jules Task 09 — TypeScript/JavaScript Parser

## Objective
Implement the TypeScript/JavaScript language parser using Tree-sitter.

## Files to Create
- `codeviz-typescript/Cargo.toml`
- `codeviz-typescript/src/lib.rs`
- `codeviz-typescript/src/parser.rs`

## Requirements
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

## Unit Tests
- Parse ESM imports and assert correct `Imports` edges
- Parse CJS `require()` and assert correct `Imports` edges
- Parse class inheritance and assert `Inherits` edge
- Parse `async function` and assert `is_async: true` on the node
