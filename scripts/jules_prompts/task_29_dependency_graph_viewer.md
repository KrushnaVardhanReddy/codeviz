TASK: T29 — DependencyGraph Viewer with Color Design System

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement the full node and edge color design system in the React Flow canvas.
Each `NodeKind` must render with its correct color, icon, and shape.
Each `EdgeKind` must render with its correct color and line style.
Add the node detail side panel that opens when a node is clicked.

Files to Modify/Create:
- `codeviz-web/components/nodes/FileNode.tsx`
- `codeviz-web/components/nodes/ModuleNode.tsx`
- `codeviz-web/components/nodes/ClassNode.tsx`
- `codeviz-web/components/nodes/InterfaceNode.tsx`
- `codeviz-web/components/nodes/FunctionNode.tsx`
- `codeviz-web/components/nodes/ConstantNode.tsx`
- `codeviz-web/components/edges/CustomEdge.tsx`
- `codeviz-web/components/DetailPanel.tsx`
- `codeviz-web/lib/colorMap.ts` (maps NodeKind/EdgeKind to color tokens)

Spec (READ ONLY — implement from it, never edit):
  docs/specs/ui/web_ui.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- CONTEXT: Use the exact color tokens defined in the spec. Do not pick your own colors.
- Extract the component into its own file (e.g. `components/DependencyGraph.tsx`) so the main page stays clean.
- Ensure the interactive diagram works fluidly.
- **CRITICAL:** Use the pre-generated static HTML and Tailwind classes from `ui-prototypes/codeviz_graph.html` to build the visual nodes and edges for the React Flow diagram. Extract the styling classes and SVG structures to ensure it matches the generated design perfectly.
- Each node component must use the color from `colorMap.ts` so colors can be changed in one place.
- Async functions must render with a ⚡ lightning badge in the top-right corner of the function node.
- The detail panel must slide in from the right (CSS transition) when a node is clicked.
- Write unit tests asserting the correct color token is returned for each NodeKind and EdgeKind.
- Ensure `npm run build` passes without errors.

═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- In React Flow, register your custom node types by passing a `nodeTypes` object to `<ReactFlow>`. Example:
  ```tsx
  const nodeTypes = { Class: ClassNode, Function: FunctionNode, ... };
  <ReactFlow nodeTypes={nodeTypes} ... />
  ```
- For custom edges, similarly pass an `edgeTypes` object with your `CustomEdge` component.
- Use Lucide React for icons: `import FileCode from 'lucide-react/icons/file-code'`.
- `colorMap.ts` should export a `NODE_COLORS` and `EDGE_COLORS` record keyed by the NodeKind/EdgeKind string values.
- The `InterfaceNode` should use a CSS `border-dashed` style to signal it's a contract, not a concrete type.
