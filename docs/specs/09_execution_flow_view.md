# Specification: Execution Flow (Entry Point) Visualization

## Goal
Introduce a secondary visualization mode that focuses on **Execution Flow** rather than architectural structure. This mode allows developers to select a specific function (an "Entry Point") and visualize a top-down flowchart of the call stack, which is critical for procedural languages, scripts, or understanding specific request life-cycles.

## User Experience (UX)

### 1. View Toggle
Add a toggle in the UI (e.g., in the Sidebar or top left of the Graph Canvas) to switch between:
- **Structural View** (Current: Classes -> Methods)
- **Execution Flow** (New: Entry Point -> Call Tree)

### 2. Entry Point Selection
When switching to Execution Flow, the canvas will initially show an **Entry Point Selector**:
- A searchable dropdown or list of functions.
- **Smart Defaults**: We will prioritize and suggest functions that act as natural entry points. For example: functions with no incoming `Calls` edges, functions named `main`, or top-level script methods.

### 3. Flowchart Interaction
Once an entry point is selected:
- The graph renders the selected function at the top (Root).
- It renders the immediate functions called by the root (1-hop callees) directly below it, connected by directed `Calls` edges.
- **In-Place Expansion**: Unlike the Structural view which completely replaces the screen when drilling down, clicking a node in the Execution Flow will **expand its callees in-place**, adding them to the tree below it. Clicking it again collapses them. This allows the user to progressively build and explore a massive call tree.

## Technical Implementation

### [MODIFY] `GraphCanvas.tsx`
- **State Additions**:
  - `primaryMode`: `'structural' | 'execution'`
  - `selectedEntryPoint`: `string | null`
  - `expandedTreeNodes`: `Set<string>` (tracks which nodes in the tree have been clicked to show their children).
- **Data Derivation**:
  - Identify root candidates: Filter `nodes` for `kind === 'Function'` where `inDegree(Calls) === 0`.
  - When in `'execution'` mode, compute `visibleNodeIds` by doing a Breadth-First Search (BFS) starting from `selectedEntryPoint`, only traversing down `Calls` edges for nodes that are in `expandedTreeNodes`.
- **Dagre Layout Tuning**:
  - Use `rankdir: 'TB'` (Top-to-Bottom) strict layout to enforce a flowchart aesthetic.
  - Disable `Inherits` and `Instantiates` edges in this mode to reduce noise; strictly show `Calls`.

### [NEW/MODIFY] Sidebar or TopNav Component
- Add a dropdown component (using a native `<select>` or custom React component) populated with the list of function nodes, sorted alphabetically, with root candidates at the top.
