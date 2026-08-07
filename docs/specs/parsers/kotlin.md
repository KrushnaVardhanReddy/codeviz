# Spec: Kotlin Parser

## File Extensions Handled
`.kt`, `.kts`

## Extraction Rules

| Source Construct | CodeGraph Output |
|---|---|
| `import com.company.Module` | `Edge { Imports }` to `com.company.Module` |
| `import com.company.*` | `Edge { Imports }` to `com.company` (package node) |
| `class Foo : Bar()` (inheritance) | `Node { Class }` + `Edge { Inherits }` |
| `class Foo : IBar` (interface impl) | `Node { Class }` + `Edge { Implements }` |
| `interface IFoo : IBar` | `Node { Interface }` + `Edge { Inherits }` |
| `fun foo()` | `Node { Function { is_async: false } }` |
| `suspend fun foo()` | `Node { Function { is_async: true } }` (suspend ≈ async) |
| `data class Foo` | `Node { Class }` with label suffix `[data]` |
| `object Foo` | `Node { Class }` with label suffix `[object]` |
| `companion object` | Skip — treat as a detail of the parent class |

## Skip Silently
- Lambda expressions
- Anonymous objects (`object : Foo() { }`)
- Inline functions (`inline fun`)

## Known Limitations
- Kotlin coroutine context (Dispatchers, scope) not resolved — out of scope.
- Extension functions (`fun String.foo()`) are extracted as standalone functions; the receiver type is not modeled as an edge.

## Tree-sitter Grammar
`tree-sitter-kotlin` — check for Kotlin 1.9 / 2.0 compatibility before use.

## Acceptance Criteria
Given:
```kotlin
import java.util.List

interface Runnable { fun run() }
open class Animal
class Dog : Animal(), Runnable {
    override fun run() {}
    suspend fun fetch() {}
}
```
Must produce:
- 1 `Imports` edge
- 1 `Interface` node (`Runnable`)
- 2 `Class` nodes (`Animal`, `Dog`)
- 1 `Inherits` edge (`Dog` → `Animal`)
- 1 `Implements` edge (`Dog` → `Runnable`)
- 2 `Function` nodes (`run`, `fetch`)
- `fetch` has `is_async: true`
