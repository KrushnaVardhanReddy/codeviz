# TypeScript & React AST Extraction Improvements

This spec outlines improvements to the Rust AST parser (`codeviz-wasm/src/lib.rs`) to properly support modern TypeScript and React idioms.

## 1. Arrow Functions in Variable Declarations
Currently, `arrow_function` nodes are ignored because their identifier is located in the parent `variable_declarator` node. 
- **Change:** We will update `extract_from_ast` to also detect `variable_declarator` nodes. If a `variable_declarator` contains an `arrow_function` as a child, we will treat the `variable_declarator` itself as a function definition.
- **Result:** Components like `export const App = () => { ... }` will correctly be identified as a Function node on the graph.

## 2. JSX Elements as Function Calls
Currently, only `call_expression` nodes are treated as function invocations. In React/TSX, component instantiations are represented as `jsx_self_closing_element` or `jsx_element`.
- **Change:** We will expand the `is_call` boolean check to include `jsx_self_closing_element` and `jsx_opening_element`.
- **Change:** When extracting the target identifier for a JSX "call", we will use the first nested `identifier` found inside the JSX node (e.g., finding `Greeter` inside `<Greeter name="World" />`).
- **Result:** React components will now show a connection (an `Instantiates` or `Calls` edge) to the child components they render.

## Proposed Changes
### `codeviz-wasm/src/lib.rs`
1. **Update `extract_from_ast`:**
   - Add logic to identify `variable_declarator` nodes containing an `arrow_function`.
   - Add `jsx_self_closing_element` and `jsx_opening_element` to the `is_call` condition.
2. **Target Identifier Extraction for Calls:**
   - If the node is a JSX element, extract the first identifier.
   - For standard calls, continue extracting the right-most identifier of member expressions.

## Verification
- Recompile the Wasm module via `make wasm`.
- Reload the playground with the default TypeScript example.
- Verify `App` appears as a node.
- Verify an edge connects `App` to `Greeter`.
