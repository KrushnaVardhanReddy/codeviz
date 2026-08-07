# Spec: Rust Parser

## File Extensions Handled
`.rs`

## Extraction Rules

| Source Construct | CodeGraph Output |
|---|---|
| `use crate::module::Item` | `Edge { Imports }` to `crate::module` |
| `use super::Item` | `Edge { Imports }` to parent module |
| `extern crate foo` | `Edge { Imports }` to `foo` |
| `struct Foo { ... }` | `Node { Class }` |
| `enum Foo { ... }` | `Node { Class }` (treated as class-like) |
| `trait Foo` | `Node { Interface }` |
| `trait Foo: Bar + Baz` | `Node { Interface }` + `Edge { Inherits }` for each supertrait |
| `impl Bar for Foo` | `Edge { Implements }` (Foo implements Bar) |
| `fn foo()` | `Node { Function { is_async: false } }` |
| `async fn foo()` | `Node { Function { is_async: true } }` |
| `pub fn` / `pub(crate) fn` | set `is_public: true` |

## Label Sanitization
Strip from all labels before storing in nodes:
- Lifetime parameters: `'a`, `'static`, etc.
- Generic angle brackets: `Vec<T>` → `Vec`
- Where clauses: do not include in label

## Skip Silently
- Macro bodies (`macro_rules!` contents, `#[proc_macro]`)
- Attribute macros on items (parse the item, ignore the attribute)
- `#[cfg(...)]` conditional compilation — parse default build only

## Workspace Support
If a `Cargo.toml` with `[workspace]` is found, treat each member crate as a sub-graph.
Each crate becomes a root `Module` node. Cross-crate `use` statements produce `Imports` edges between crates.

## Tree-sitter Grammar
`tree-sitter-rust` — excellent quality, handles Rust 2021 edition.

## Acceptance Criteria
Given:
```rust
use std::fmt;

trait Greet: fmt::Display {}
struct Dog;
impl Greet for Dog {}
pub async fn bark() {}
```
Must produce:
- 1 `Imports` edge (`std::fmt`)
- 1 `Interface` node (`Greet`)
- 1 `Inherits` edge (`Greet` → `fmt::Display`)
- 1 `Class` node (`Dog`)
- 1 `Implements` edge (`Dog` → `Greet`)
- 1 async `Function` node (`bark`) with `is_public: true`
