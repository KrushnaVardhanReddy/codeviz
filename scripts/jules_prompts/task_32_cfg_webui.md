TASK: T32 — CFG Renderer in Web UI Side Panel

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Add a Control Flow Graph renderer inside the Web UI's detail side panel.
When a user clicks on a `Function` node, the CFG for that function is rendered
as a mini React Flow graph inside the side panel using the CFG visual design system.

Files to Modify/Create:
- `codeviz-web/components/CfgPanel.tsx` (CFG React Flow mini-canvas)
- `codeviz-web/components/nodes/CfgBlockNode.tsx` (renders a single CFG block)
- `codeviz-web/lib/cfgColorMap.ts` (maps CfgBlockKind/CfgEdgeKind to colors)
- `codeviz-web/components/DetailPanel.tsx` (wire in CfgPanel below node details)

Spec (READ ONLY — implement from it, never edit):
  docs/specs/ui/control_flow_graph.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- CONTEXT: The CFG panel should use a nested `<ReactFlow>` instance inside the side panel. Give it a fixed height (e.g., 400px).
- Each `CfgBlockKind` must render with the correct color and shape from the spec.
- Diamond shapes for `Condition` and `LoopHeader` blocks must be rendered using CSS `transform: rotate(45deg)` on the node wrapper.
- Edge labels ("✓ true", "✗ false") must be visible on the edges.
- If a function has no CFG data (`control_flow` is null/undefined), show a placeholder: "CFG not available for this function."
- Write unit tests for `cfgColorMap.ts`.
- Ensure `npm run build` passes without errors.

═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- React Flow supports nested graphs natively. Use `fitView` on the nested `<ReactFlow>` in the CFG panel so the graph auto-fits its container height.
- For diamond shapes, the cleanest approach is to have the node's outer `div` be square (e.g., 80x80px) with `transform: rotate(45deg)` and then counter-rotate the inner text with `transform: rotate(-45deg)`.
- For `TrueBranch` edges, use green color and `label="✓ true"`. For `FalseBranch`, use red and `label="✗ false"`.
- `LoopBack` edges should use `type="selfConnecting"` or a custom curved path to visually curve back to the loop header.
- `cfgColorMap.ts` mirrors `colorMap.ts` but for CFG block kinds and edge kinds.
