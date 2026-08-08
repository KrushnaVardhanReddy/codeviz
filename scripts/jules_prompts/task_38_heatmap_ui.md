TASK: T38 — Heatmap UI Layer

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Integrate `churn_score` and `primary_authors` into the React Flow Web UI to show "Hotspots".

Files to Modify/Create:
- `codeviz-web/components/Toolbar.tsx` (add toggle)
- `codeviz-web/components/nodes/FileNode.tsx` (apply heatmap colors)
- `codeviz-web/components/DetailPanel.tsx` (show authors)

Spec (READ ONLY — implement from it, never edit):
  docs/specs/ui/heatmap.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- Toggle must switch node backgrounds from standard colors to a heatmap scale (blue to red based on churn).
- Do not break existing color modes.

═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- Use a simple CSS scale or D3 color scale for mapping churn values to colors.
