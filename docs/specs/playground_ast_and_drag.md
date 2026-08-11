# Playground AST & Drag Improvements (T63)

## Overview
This specification details the next evolution of the web-based Code Playground for CodeViz. While MVP v1 successfully parses flat function definitions and procedural calls, it falls short when dealing with Object-Oriented patterns (e.g., class inheritance, method invocations) and UI interactivity (React Flow nodes are statically positioned and cannot be dragged).

## 1. Deep AST Parsing (`codeviz-wasm/src/lib.rs`)

The `extract_from_ast` function must be enhanced to properly extract classes and method calls.

### Inheritance Edges
When encountering a `class_definition` (or `class_declaration`), the parser must look for an `argument_list` or `class_heritage` child node to identify base classes. 
- For each inherited class identifier found, construct a `to_id` (e.g. `file_path::BaseClass`).
- Push an `EdgeKind::Inherits` edge from the current class to the base class.

### Method Invocations
When processing a `call` or `call_expression` node, the target identifier is often deeply nested inside an `attribute` or `member_expression` node (e.g. `d.bark()` in Python is represented as a `call` wrapping an `attribute` which contains the identifiers `d` and `bark`).
- The parser must recursively search the left-hand side of the call expression to find the right-most identifier representing the method being invoked.
- Push an `EdgeKind::Calls` edge from the `current_scope_id` to the invoked method (e.g. `file_path::bark`).

## 2. React Flow Drag-and-Drop (`codeviz-web/components/GraphCanvas.tsx`)

Currently, `nodesToRender` and `edgesToRender` are fully re-calculated using `useMemo` on every keystroke in the playground, anchoring nodes to hardcoded `(x, y)` grid coordinates.

### Implementation Requirements
- Replace the raw mapped coordinates with `useNodesState` and `useEdgesState` hooks provided by `@xyflow/react`.
- When the `graph` prop changes, gracefully update the internal nodes state. New nodes should be placed on a grid (or via a simple layout algorithm), but existing nodes should preserve their dragged `(x, y)` positions.
- Ensure the `onNodesChange` and `onEdgesChange` callbacks are wired up to the `<ReactFlow>` component so users can freely drag nodes around the canvas to reorganize the architecture view.
