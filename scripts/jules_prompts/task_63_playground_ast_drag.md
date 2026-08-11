# Task 63: Playground AST & Node Dragging Improvements

This task addresses two primary issues discovered during MVP v1 E2E testing of the interactive Code Playground:
1. The Rust AST parser in the WASM package fails to extract edges for object-oriented structures (class inheritance and method calls).
2. The React Flow canvas locks nodes in place, preventing users from dragging nodes to reorganize the architecture.

## CORE SPECS (READ ONLY — understand the requirements):
- `docs/specs/wasm_basic_call_edges.md` (Explains the basic heuristic currently in place).
- `docs/specs/playground_ast_and_drag.md` (Explains exactly what you need to implement for this task).

## FILES TO MODIFY:

### 1. `codeviz-wasm/src/lib.rs` (AST Parsing)
- The `extract_from_ast` function currently detects simple calls, but you need to expand it:
  - **Inheritance**: Detect class definitions and look for `argument_list` (Python) or `class_heritage` (TS) to extract base classes and push `EdgeKind::Inherits`.
  - **Method Calls**: Modify the `call` detection logic. If the call target is wrapped in an `attribute` (Python) or `member_expression` (TS), recursively traverse it to extract the rightmost `identifier` or `property_identifier` (e.g., extracting "bark" from `d.bark()`).

### 2. `codeviz-web/components/GraphCanvas.tsx` (React Flow Dragging)
- Currently, `rawNodes` maps the JSON graph to hardcoded positions. `nodesToRender` recalculates these positions on every render.
- You must refactor this to use `@xyflow/react` hooks: `useNodesState` and `useEdgesState`.
- Maintain a stable state for node positions. When `graph.nodes` changes (e.g., a user types new code), add new nodes but preserve the `(x, y)` coordinates of existing nodes so they aren't reset when dragged.
- Wire up `onNodesChange` and `onEdgesChange` to the `<ReactFlow>` component.

## ACCEPTANCE CRITERIA:
1. **Rust Tests**: Add simple unit tests inside `codeviz-wasm/src/lib.rs` that verify the AST parser correctly produces `Inherits` edges for classes and `Calls` edges for method invocations.
2. **WASM Compilation**: Ensure the Rust code compiles cleanly for the web target.
3. **UI Dragging**: The React Flow nodes must be fully draggable, and typing in the Monaco editor should not snap dragged nodes back to their initial grid coordinates.
