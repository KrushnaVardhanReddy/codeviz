TASK: T54 — OpenTelemetry Trace Overlay

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement the ability to import an OpenTelemetry (OTEL) trace JSON file and
overlay its runtime execution path directly onto the CodeViz architecture graph.
This provides runtime context on top of static architecture analysis.

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/otel_trace_overlay.md

═══════════════════════════════════════════════════════════════
BACKGROUND — WHAT ALREADY EXISTS
═══════════════════════════════════════════════════════════════

- The CLI entry point is `codeviz-cli/src/main.rs`.
- Graph generation logic is in `codeviz-core/src/graph.rs` and `codeviz-core/src/render/`.
- The Web UI routes are in `codeviz-web/app/`.
- The `GraphCanvas` component in `codeviz-web/components/GraphCanvas.tsx` can accept custom node styling.

═══════════════════════════════════════════════════════════════
DELIVERABLES
═══════════════════════════════════════════════════════════════

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. CREATE: codeviz-core/src/trace.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Define structs to parse a standard OTEL trace JSON (Jaeger/Zipkin compatible format):
  { "traceID": "...", "spans": [ { "operationName": "...", "duration": 123, ... } ] }

Implement a matching algorithm:
- Try exact match: `span.operationName == node.label`
- Fallback fuzzy match: `span.operationName` contains `node.label` (case-insensitive)

Register `trace` module in `codeviz-core/src/lib.rs`.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
2. MODIFY: codeviz-core/src/render/mermaid.rs & html.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Extend the rendering logic to support an optional trace overlay.
- Matched nodes in Mermaid should have a specific class (e.g. `classDef hot fill:#ff9900`).
- HTML output should include the raw trace data so the timeline scrubber can use it.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
3. MODIFY: codeviz-cli/src/main.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Add a new `trace` subcommand:
```bash
codeviz trace --path <dir> --otel <trace.json> --output trace_overlay.html
```
If no matches are found, it must not crash — it should print a warning and render the base graph.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
4. MODIFY: codeviz-web/app/w/[slug]/repos/[repo]/trace/page.tsx (or similar route)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Implement the web UI for trace overlay:
- An "Import OTEL Trace" upload button.
- A timeline scrubber component below the graph canvas.
- Highlight the nodes on the `GraphCanvas` when a trace is loaded.

═══════════════════════════════════════════════════════════════
CRITICAL IMPLEMENTATION RULES
═══════════════════════════════════════════════════════════════
1. This is an `[ENT]` tier feature. If there is a license check, gate it behind the Enterprise plan.
2. The matching algorithm must gracefully handle zero matches.
3. No heavy external dependencies for OTEL parsing; just use `serde_json` to parse the structure.
4. Ensure `cargo test --all` and `cargo clippy --all -- -D warnings` pass.

Commit: "jules: T54 — OpenTelemetry trace overlay on architecture graph"
Target branch: feat-t54-otel-trace
