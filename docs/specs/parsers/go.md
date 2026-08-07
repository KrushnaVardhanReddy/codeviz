# Spec: Go Parser

## File Extensions Handled
`.go`

## Extraction Rules

| Source Construct | CodeGraph Output |
|---|---|
| `import "pkg/path"` | `Edge { Imports }` to `pkg/path` |
| `import ( "a"; "b" )` block | One `Edge { Imports }` per package |
| `import alias "pkg/path"` | `Edge { Imports }` to `pkg/path` (alias ignored) |
| `type Foo struct { Bar }` (embedded) | `Node { Class }` + `Edge { Inherits }` to `Bar` |
| `type Foo interface { }` | `Node { Interface }` |
| `func Foo() {}` | `Node { Function }` (package-level) |
| `func (f *Foo) Method() {}` | `Node { Function }` associated with struct `Foo` |
| `func main()` | `Node { Function }` marked as entry point (`is_public: true`) |

## Special: go.mod
Read `go.mod` at the source root to resolve the module name.
Use the module name to canonicalize import paths:
- `"github.com/user/myapp/pkg/parser"` → normalized to `pkg/parser` relative to source root.

## Skip Silently
- Blank imports `import _ "pkg"` (side-effect only)
- Build tag lines `//go:build ...`
- `cgo` imports (`import "C"`)

## Known Limitations
- Interface satisfaction is **implicit** in Go. Do NOT attempt to infer `Implements` edges.
  Document this limitation with a code comment.
- `go generate` directives are skipped.

## Tree-sitter Grammar
`tree-sitter-go` — very stable, handles Go 1.21+.

## Acceptance Criteria
Given:
```go
package main

import (
    "fmt"
    "myapp/utils"
)

type Runner interface { Run() }
type Dog struct { Animal }
func (d *Dog) Run() {}
func main() {}
```
Must produce:
- 2 `Imports` edges (`fmt`, `myapp/utils`)
- 1 `Interface` node (`Runner`)
- 1 `Class` node (`Dog`)
- 1 `Inherits` edge (`Dog` → `Animal`)
- 2 `Function` nodes (`Run`, `main`)
- `main` marked as entry point
