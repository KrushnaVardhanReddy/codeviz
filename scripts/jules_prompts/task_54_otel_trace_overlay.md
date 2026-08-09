TASK: T54 — OpenTelemetry Trace Overlay

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Import an OpenTelemetry trace JSON and overlay the actual runtime
execution path onto the CodeViz graph, with timing data and a
timeline scrubber.

Files to Create/Modify:
- `codeviz-web/app/w/[slug]/repos/[repo]/trace/page.tsx` [NEW]
- `codeviz-web/components/TraceTimeline.tsx` [NEW]
- `codeviz-web/lib/otelParser.ts` [NEW]
- `codeviz-cli/src/main.rs` (add `trace` subcommand for offline HTML output)

Spec (READ ONLY):
  docs/specs/features/otel_trace_overlay.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- This is a [BIZ] tier feature — add plan-check guard on the API route.
- Must gracefully handle zero span-to-node matches (show warning).
- Fuzzy matching: case-insensitive contains match as fallback.
- `npm run build` and `cargo test --all` must pass.
