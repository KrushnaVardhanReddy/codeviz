TASK: T53 — Interactive Call Path Explorer (Web UI)

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Add an animated, interactive call path explorer to the CodeViz Web UI.
Clicking a Function node triggers a BFS animation showing all reachable
call paths, step-by-step.

Files to Create/Modify:
- `codeviz-web/components/CallPathExplorer.tsx` [NEW]
- `codeviz-web/hooks/usePathAnimation.ts` [NEW]
- `codeviz-web/components/DetailPanel.tsx` [MODIFY]  (add "Trace Paths" button)

Spec (READ ONLY):
  docs/specs/features/call_path_explorer.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- Works entirely client-side. No new API routes.
- Animation uses requestAnimationFrame — must not block the UI thread.
- "Trace Paths" button only visible when a Function node is selected.
- Write unit tests for `usePathAnimation` BFS logic.
- `npm run build` must pass.
