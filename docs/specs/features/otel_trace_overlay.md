# Spec: OpenTelemetry Trace Overlay (Phase 22)

## Overview
Import an OpenTelemetry (OTEL) trace JSON file and overlay the actual runtime
execution path onto the CodeViz architecture graph. Shows which functions were
ACTUALLY called for a specific request, with timing data.

## CLI Usage
```bash
codeviz trace --path . --otel trace.json --output trace_overlay.html
```

## Input Format
Accepts standard OTEL trace JSON (Jaeger/Zipkin compatible):
```json
{
  "traceID": "...",
  "spans": [
    { "operationName": "parse_file", "duration": 142, "references": [...] }
  ]
}
```

## Matching Strategy
1. Exact match: `span.operationName` == `Node.label`
2. Fuzzy match: `span.operationName` contains `Node.label` (case-insensitive)
3. Unmatched spans are shown in a separate "Unmapped Spans" list.

## Overlay Rendering
- Matched nodes are highlighted in a distinct "hot" color (orange/red).
- Edge thickness is proportional to number of times that edge was traversed.
- A timeline scrubber lets the user replay the trace chronologically.
- Hovering a node shows `span.duration` as a tooltip.

## Web UI Integration
- New route: `codeviz-web/app/w/[slug]/repos/[repo]/trace/page.tsx`
- Upload button: "Import OTEL Trace" in the graph toolbar.
- Timeline scrubber component below the graph canvas.

## Constraints
- `[BIZ]` tier — gate behind Business plan check.
- CLI output mode must work offline (no web UI required).
- Matching must gracefully handle zero matches (show warning, not crash).
- `cargo test --all` / `npm run build` must pass.
