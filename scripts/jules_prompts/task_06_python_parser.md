# Jules Task 06 — Python Parser (Imports & Classes)

## Objective
Implement the Python language parser in a new `codeviz-python` crate using Tree-sitter.

## Files to Create
- `codeviz-python/Cargo.toml`
- `codeviz-python/src/lib.rs`
- `codeviz-python/src/parser.rs`
- Update workspace `Cargo.toml` to include this crate

## Requirements
1. Add `tree-sitter` and `tree-sitter-python` as dependencies.
2. Implement `LanguageParser` for a `PythonParser` struct.
3. Extract the following into `CodeGraph`:
   - `import X` and `from X import Y` → `Edge { kind: EdgeKind::Imports }`
   - `class Foo(Bar):` → `Node { kind: NodeKind::Class }` + `Edge { kind: EdgeKind::Inherits }`
   - `def foo():` → `Node { kind: NodeKind::Function { is_async } }`
   - `@decorator` → add to node label as suffix
4. Dynamic imports (`importlib.import_module`) — skip silently, don't panic.
5. Circular imports — add both edges normally, let the renderer handle display.

## Unit Tests
Parse the following snippet and assert the resulting `CodeGraph` has:
- 2 `Imports` edges (os, pathlib.Path)
- 2 `Class` nodes (Animal, Dog)
- 1 `Inherits` edge (Dog → Animal)
- 1 async `Function` node (main)

```python
import os
from pathlib import Path

class Animal:
    pass

class Dog(Animal):
    def bark(self): pass

async def main():
    d = Dog()
```


═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- Use stable Rust 2021 edition. Do NOT use unstable features like `let_chains`.
- Always run `cargo test --all` and `cargo clippy --all -- -D warnings` before completing the task.
