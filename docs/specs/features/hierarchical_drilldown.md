# Spec: Hierarchical Drill-Down & Scope Resolution (Phase 22 / MVP v2)

## Overview
To handle large repositories without the "hairball" effect, CodeViz will implement a Hierarchical Drill-Down UI strategy inspired by tools like Structure101. Instead of rendering all nodes at once, the UI will group Functions and Classes into their parent Modules, allowing users to interactively expand them. This requires the backend parsers (starting with Python) to accurately extract parent-child boundaries (`parent_id`) and perfectly resolve function calls using local scope.

## Requirements

### Backend: Python Scope Resolution (`codeviz-python`)
- **Hierarchical Nesting**: Extract `parent_id` for functions and classes. Emit `EdgeKind::Contains` from parent to child.
- **Call Resolution**: Implement a local scope map during AST traversal.
  - Track `import` and `from ... import` statements.
  - Track local function/class definitions.
  - Resolve calls like `self.method()` to the current class.
  - Resolve calls like `func()` to the local scope or imports.
  - Unresolved calls are marked as external/constant edges.

### Frontend: Interactive Drill-Down (`codeviz-web`)
- **Default State**: Render only `File` and `Module` nodes. `Class` and `Function` nodes are hidden.
- **Interaction**: Double-clicking a compound node (Module/Class) expands it, revealing its children.
- **Edge Aggregation (Roll-up)**: If a hidden function calls another hidden function in a different module, the edge must visually "roll up" to connect the visible parent modules instead. This reduces visual clutter significantly.

## Acceptance Criteria
1. The `codeviz-python` parser sets `parent_id` correctly on nodes.
2. The `codeviz-python` parser correctly identifies function calls and resolves their targets via scope.
3. The React Flow UI (`GraphCanvas.tsx`) successfully collapses nested nodes and reroutes edges to the parent node.
