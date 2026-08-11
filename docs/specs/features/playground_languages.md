# Spec: Playground Additional Languages — T64

## Overview
The Interactive Code Playground (`http://localhost:3003/playground`) currently only supports Python and TypeScript. Since the core backend already supports Go, Rust, Java, and Kotlin, the web playground should also support these languages natively in the browser using their respective WebAssembly (WASM) tree-sitter grammars.

## Requirements
1. **Frontend UI Update (`codeviz-web/components/PlaygroundLayout.tsx`)**:
   - Update the `Language` dropdown to include `go`, `rust`, `java`, and `kotlin`.
   - Add default code snippets for each new language in the `EXAMPLES` constant (e.g., a simple class/struct and function instantiation).
   - Ensure the WASM file name logic maps correctly (e.g., `rust` -> `tree-sitter-rust.wasm`, `go` -> `tree-sitter-go.wasm`). All required `.wasm` files are already present in `codeviz-web/public/tree-sitter-wasms/`.

2. **WASM AST Extraction (`codeviz-wasm/src/lib.rs`)**:
   - The current `extract_from_ast` function relies on generic tree-sitter node types like `function_definition`, `class_definition`, and `call_expression`.
   - Update this logic to accurately extract nodes for the new languages:
     - **Rust**: Add support for `struct_item`, `impl_item`, `function_item`.
     - **Go**: Add support for `type_declaration` (structs/interfaces), `method_declaration`, `function_declaration`.
     - **Java/Kotlin**: Add support for `class_declaration`, `method_declaration`, `object_declaration` (Kotlin).
   - Edges (Calls, Instantiates, Contains) should continue to be mapped appropriately based on the language's AST structure.

3. **Validation**:
   - The playground should successfully parse the default examples for Go, Rust, Java, and Kotlin.
   - The React Flow graph should correctly visualize the classes/structs and functions, including the nested `Contains` edges.

## Constraints
- Do not add new Rust crates or npm dependencies.
- Handle node parsing defensively to prevent WASM panics if a language's AST uses unexpected nodes.
