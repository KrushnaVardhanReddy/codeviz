# Spec: CodeViz Web UI (Phase 11)

## Overview
A free, open-source interactive web application built with **Next.js** and **React Flow**
that visualizes the `CodeGraph` JSON produced by the CodeViz CLI or WASM engine.
It runs entirely in the browser (no backend required) by loading the `codeviz-wasm`
package built in Task 24.

## Technology Stack
- **Framework:** Next.js 14 (App Router)
- **Graph Library:** React Flow (`@xyflow/react`)
- **Styling:** Tailwind CSS
- **WASM integration:** `codeviz-wasm` npm package (Task 24)
- **Icons:** Lucide React

---

## Zoom Levels (Hierarchical Drill-Down)

### Level 1 — Bird's Eye View
- Only `File` and `Module` nodes are visible.
- The graph is clean, minimal, and easy to read even for large codebases.
- Clicking a Module node expands it to Level 2.

### Level 2 — Architecture View (Default)
- `Class`, `Interface`, and top-level `Function` nodes appear inside their
  parent module as sub-nodes.
- Edges are visible and color-coded by type (see Edge Design).
- Clicking a Function/Class expands it to Level 3.

### Level 3 — Function Detail
- Clicking a `Function` node opens a **side panel** showing:
  1. The function's source code snippet.
  2. Its Control Flow Graph (Phase 12 feature — stub panel with "CFG coming soon"
     label in Phase 11).
  3. All inbound and outbound edges.

---

## Visual Design System

### Node Colors (by NodeKind)

| NodeKind       | Background   | Border       | Icon | Shape             |
|----------------|-------------|-------------|------|-------------------|
| `File`         | #1E3A5F (dark blue)   | #3B82F6 (blue)   | 📄 | Rounded rect |
| `Module`       | #2D1B69 (dark purple) | #8B5CF6 (purple) | 📦 | Rect with header |
| `Class`        | #7C2D12 (dark orange) | #F97316 (orange) | 🔷 | Rect bold border |
| `Interface`    | #713F12 (dark yellow) | #EAB308 (yellow) | ◇  | Dashed rect  |
| `Function`     | #14532D (dark green)  | #22C55E (green)  | ƒ  | Pill/rounded |
| `Async Fn`     | #14532D (dark green)  | #22C55E (green)  | ⚡ | Pill + lightning badge |
| `Constant`     | #1F2937 (dark gray)   | #6B7280 (gray)   | π  | Small rect   |

### Edge Colors (by EdgeKind)

| EdgeKind      | Color                | Line Style        | Arrow     |
|---------------|---------------------|------------------|-----------|
| `Imports`     | #3B82F6 (blue)      | Solid             | Open arrow |
| `Calls`       | #22C55E (green)     | Solid             | Filled arrow |
| `Inherits`    | #F97316 (orange)    | Solid, 2px thick  | Hollow triangle |
| `Implements`  | #EAB308 (yellow)    | Dashed            | Open arrow |
| `Returns`     | #6B7280 (gray)      | Dotted, thin      | Dotted arrow |
| `Instantiates`| #8B5CF6 (purple)    | Solid             | Diamond head |

### Background
- Dark mode: `#0F172A` (Tailwind `slate-900`)
- Grid pattern for depth effect: `#1E293B` (slate-800) dot grid

---

## UI Layout

```
┌──────────────────────────────────────────────────────────────┐
│  🔍 Search nodes   [Python ▼]  [Module ▼]  [Export JSON]    │ ← Toolbar
├─────────────────────────────────┬────────────────────────────┤
│                                 │                            │
│        React Flow Canvas        │     Detail Side Panel      │
│      (full height, dark bg)     │  (opens when node clicked) │
│                                 │  - Node name & kind        │
│                                 │  - Source code snippet     │
│                                 │  - All connected edges     │
│                                 │  - CFG (Phase 12)          │
├─────────────────────────────────┴────────────────────────────┤
│ Legend: 🔷Class  ◇Interface  ƒFunction  📦Module  📄File    │ ← Legend bar
│         ── Imports  ── Calls  ── Inherits  ·· Returns       │
└──────────────────────────────────────────────────────────────┘
```

---

## Input Methods
1. **Paste JSON:** User pastes a `CodeGraph` JSON blob directly.
2. **Drop JSON file:** Drag and drop a `.json` file exported by `codeviz export --format json`.
3. **WASM Parse (future):** Upload source files; browser parses them inline using the WASM engine.

---

## Acceptance Criteria
- [ ] Renders a `CodeGraph` JSON with all node types and edge types.
- [ ] Each `NodeKind` renders with the correct color and icon.
- [ ] Each `EdgeKind` renders with the correct color and line style.
- [ ] Clicking a node highlights all its connected edges.
- [ ] Clicking a `Module` node collapses/expands its children.
- [ ] A legend is always visible.
- [ ] Dark mode by default.
- [ ] Ships as a standalone Next.js app deployable to Vercel.
