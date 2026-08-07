# Spec: TypeScript / JavaScript Parser

## File Extensions Handled
`.ts`, `.tsx`, `.js`, `.jsx`, `.mjs`, `.cjs`

## Extraction Rules

| Source Construct | CodeGraph Output |
|---|---|
| `import { X } from 'y'` (ESM) | `Edge { Imports }` to module `y` |
| `import * as X from 'y'` | `Edge { Imports }` to module `y` |
| `import 'y'` (side-effect) | `Edge { Imports }` to module `y` |
| `const x = require('y')` (CJS) | `Edge { Imports }` to module `y` |
| `class Foo extends Bar` | `Node { Class }` + `Edge { Inherits }` |
| `interface IFoo extends IBar` | `Node { Interface }` + `Edge { Inherits }` |
| `function foo()` | `Node { Function { is_async: false } }` |
| `async function foo()` | `Node { Function { is_async: true } }` |
| `const foo = () => {}` | `Node { Function }` with label `foo` |
| `export default` / `export { X }` | set `is_public: true` on node |

## Skip Silently
- `import('...')` — dynamic import
- `require.resolve(...)` — meta usage
- Type-only imports (`import type { X }`) — skip entirely (no runtime dependency)

## Known Limitations
- Barrel file (`index.ts`) re-exports are not expanded in V0.2
- Module path aliases (e.g., `@/components`) are not resolved

## Tree-sitter Grammar
`tree-sitter-typescript` — handles TS, TSX, JS, JSX.
Use the `tsx` grammar variant for `.tsx` and `.jsx` files.

## Acceptance Criteria
Given:
```typescript
import { readFile } from 'fs';
import type { Config } from './config';

class Animal {}
class Dog extends Animal {
  async fetch(): Promise<void> {}
}
```
Must produce:
- 1 `Imports` edge (to `fs`; the type-only import is skipped)
- 2 `Class` nodes
- 1 `Inherits` edge
- 1 async `Function` node (`fetch`)
