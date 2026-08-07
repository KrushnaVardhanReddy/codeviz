# Spec: Java Parser

## File Extensions Handled
`.java`

## Extraction Rules

| Source Construct | CodeGraph Output |
|---|---|
| `import com.company.Module` | `Edge { Imports }` to `com.company.Module` |
| `import com.company.*` | `Edge { Imports }` to `com.company` (package node) |
| `class Foo extends Bar` | `Node { Class }` + `Edge { Inherits }` |
| `class Foo implements IBar, IBaz` | `Node { Class }` + one `Edge { Implements }` per interface |
| `interface IFoo extends IBar` | `Node { Interface }` + `Edge { Inherits }` |
| `public void method()` | `Node { Function }` |
| `public static void main(String[] args)` | `Node { Function }` marked as entry point |
| `@Override`, `@Autowired`, etc. | Store as metadata on node label suffix `[@Override]` |

## Skip Silently
- Anonymous classes (`new Foo() { ... }`)
- Lambda expressions (`x -> x + 1`)
- Local variable declarations
- Static initializer blocks

## Known Limitations
- Annotation processors (`@Bean`, Spring context) are not resolved. Document this.
- Wildcard imports produce a package-level node, not per-class nodes.

## Tree-sitter Grammar
`tree-sitter-java` — stable, handles Java 17 features.

## Acceptance Criteria
Given:
```java
import java.util.List;
import java.io.*;

interface Runnable { void run(); }
class Animal {}
class Dog extends Animal implements Runnable {
    @Override
    public void run() {}
    public static void main(String[] args) {}
}
```
Must produce:
- 2 `Imports` edges (`java.util.List`, `java.io`)
- 1 `Interface` node (`Runnable`)
- 2 `Class` nodes (`Animal`, `Dog`)
- 1 `Inherits` edge (`Dog` → `Animal`)
- 1 `Implements` edge (`Dog` → `Runnable`)
- 2 `Function` nodes (`run`, `main`)
- `main` marked as entry point
