# Spec: Interactive Call Path Explorer (Phase 20)

## Overview
An animated, interactive execution flow explorer built into the CodeViz Web UI.
Users click any function node and watch the call graph animate step-by-step,
showing all reachable paths from that entry point.

## How It Works
1. User clicks a `Function` node in the React Flow graph.
2. A "Trace Paths" button appears in the Detail Panel (right sidebar).
3. On click, the graph animates breadth-first from that node:
   - Nodes light up in sequence (using a pulsing highlight animation).
   - Edge arrows animate to show directionality.
   - Each hop pauses for 400ms before proceeding to the next level.
4. A step counter shows "Step 2 of 7 — `dispatch → python_parser`".
5. User can pause, step forward/backward, or reset.

## UI Components to Create/Modify
- `codeviz-web/components/CallPathExplorer.tsx` [NEW]
  - Controls: Play / Pause / Step Forward / Step Back / Reset
  - Step counter and current node label
- `codeviz-web/components/DetailPanel.tsx` [MODIFY]
  - Add "Trace Paths" button when selected node is a Function
- `codeviz-web/hooks/usePathAnimation.ts` [NEW]
  - BFS traversal logic over the React Flow nodes/edges state
  - Returns animation frames as a sequence of `Set<nodeId>`

## Data Source
Uses the existing React Flow graph state — no new API calls needed.
BFS traversal is done client-side on `edges` filtered by `EdgeKind::Calls`.

## Constraints
- Works entirely client-side. No new API routes needed.
- Animation must be pausable and not block the UI thread (use `requestAnimationFrame`).
- Selecting a non-Function node hides the "Trace Paths" button.
- `npm run build` must pass.
- Write unit tests for `usePathAnimation` BFS logic.
