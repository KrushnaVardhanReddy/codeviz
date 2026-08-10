TASK: T38 — Enterprise Insights: Heatmap UI

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement a Heatmap view in the CodeViz Web UI. This feature allows users to toggle
different color overlays on the React Flow graph canvas based on `churn_score` and
`health_score` (from T36). This visually surfaces technical debt and high-risk hotspots.

═══════════════════════════════════════════════════════════════
BACKGROUND — WHAT ALREADY EXISTS
═══════════════════════════════════════════════════════════════
- The Web UI relies on React Flow in `codeviz-web/components/GraphCanvas.tsx`.
- The `NodeMeta` contains fields like `churn_score` and `health_score`.

═══════════════════════════════════════════════════════════════
DELIVERABLES
═══════════════════════════════════════════════════════════════

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. MODIFY: codeviz-web/components/GraphCanvas.tsx
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Add a "View Mode" dropdown/toggle in the toolbar above the canvas:
- Standard (Default colors based on node kind)
- Git Churn Heatmap (Colors nodes based on `churn_score` relative to max churn in the graph: cool blue to hot red)
- Health Score Heatmap (Colors nodes based on `health_score`: green > 8, yellow 5-8, red < 5)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
2. CREATE/MODIFY: Node Components
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Update the custom React Flow node component (`codeviz-web/components/CustomNode.tsx` or similar)
to accept a `viewMode` prop or read it from a React Context/Zustand store.
Apply the appropriate background colors based on the mode.
Ensure text remains readable (e.g. use white text on dark red backgrounds).

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
3. ADD: Legend Component
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
When a heatmap mode is active, display a floating legend in the bottom right corner
explaining the color scale.

═══════════════════════════════════════════════════════════════
CRITICAL IMPLEMENTATION RULES
═══════════════════════════════════════════════════════════════
1. Do not break standard node selection or drag/drop functionality.
2. The logic for calculating max/min for the color scale should happen dynamically based on the current graph's nodes.
3. Run `npm run build` to ensure no TypeScript compilation errors.

Commit: "jules: T38 — Graph Heatmap UI for Git churn and Health scores"
Target branch: feat-t38-heatmap-ui
