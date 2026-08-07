# Spec: Python Parser

## Extraction Rules

| Source Construct | CodeGraph Output |
|---|---|
| `import X` | `Edge { from: current_file, to: "X", kind: Imports }` |
| `from X import Y` | `Edge { from: current_file, to: "X", kind: Imports }` |
| `class Foo:` | `Node { kind: Class, label: "Foo" }` |
| `class Foo(Bar, Baz):` | `Node { Class }` + `Edge { Inherits }` for each base |
| `def foo():` | `Node { Function { is_async: false } }` |
| `async def foo():` | `Node { Function { is_async: true } }` |
| `@decorator` | append `[@decorator]` to node label |

## Skip Silently (No Error)
- `importlib.import_module(...)` — dynamic import, unresolvable
- `__import__(...)` — dynamic import
- C extension modules (`import _foo`) — skip if import fails resolution
- Type-checking-only imports (`if TYPE_CHECKING: import X`)

## Known Limitations (Document in Code Comments)
- Function calls between functions are **not** extracted (no type inference)
- `__all__` is not used to filter public surface in V0.1
- Circular imports produce duplicate edges — the renderer deduplicates

## Tree-sitter Grammar
`tree-sitter-python` — stable, no known issues with Python 3.12 syntax.

## Acceptance Criteria
Given this input:
```python
import os
from pathlib import Path

class Animal: pass

class Dog(Animal):
    def bark(self): pass

async def main():
    d = Dog()
```
The resulting `CodeGraph` must contain:
- Exactly 2 `Imports` edges (to `os` and `pathlib`)
- Exactly 2 `Class` nodes (`Animal`, `Dog`)
- Exactly 1 `Inherits` edge (`Dog` → `Animal`)
- Exactly 1 `Function` node (`bark`) inside `Dog`
- Exactly 1 async `Function` node (`main`)
