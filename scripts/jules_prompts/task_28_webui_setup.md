TASK: T28 — Next.js + React Flow Web UI Setup

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Scaffold a new Next.js 14 (App Router) project at `codeviz-web/` inside the repo.
Set up React Flow, Tailwind CSS, and a dark-mode canvas ready to render a CodeGraph.

Files to Create:
- `codeviz-web/` (Next.js project root)
- `codeviz-web/package.json`
- `codeviz-web/app/page.tsx` (main page with React Flow canvas)
- `codeviz-web/app/layout.tsx` (root layout, dark mode)
- `codeviz-web/components/GraphCanvas.tsx` (React Flow wrapper)
- `codeviz-web/components/Legend.tsx` (edge/node color legend)
- `codeviz-web/lib/graphTypes.ts` (TypeScript types mirroring CodeGraph IR)

Spec (READ ONLY — implement from it, never edit):
  docs/specs/ui/web_ui.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- CONTEXT: Use Next.js 14 App Router (not Pages Router). Use TypeScript throughout.
- Do NOT use `create-react-app`. Scaffold with `npx create-next-app@latest codeviz-web --typescript --tailwind --app --no-src-dir`.
- The canvas background must be dark (`slate-900`) with a dot-grid pattern.
- The Legend component must be always visible in the bottom-left corner of the canvas.
- Write unit tests for the `graphTypes.ts` TypeScript interfaces.
- Ensure `npm run build` passes without errors.

═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- Install React Flow with: `npm install @xyflow/react`.
- For the dot-grid background, use React Flow's built-in `<Background variant="dots" />` component.
- For dark mode, add `darkMode: 'class'` to `tailwind.config.ts` and set `<html className="dark">` in layout.tsx.
- The `graphTypes.ts` file should define TypeScript interfaces that exactly mirror the Rust `CodeGraph`, `Node`, `Edge`, `NodeKind`, and `EdgeKind` structs so the JSON import is type-safe.
- Start with a hardcoded sample `CodeGraph` JSON in `page.tsx` so the canvas renders immediately without needing the WASM engine yet.
