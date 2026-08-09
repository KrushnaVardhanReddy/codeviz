# Jules Task 04 — Mermaid Renderer

## Objective
Build the Mermaid diagram renderer in `codeviz-core`. Takes a `CodeGraph`, outputs a Mermaid string.

## Files to Create/Modify
- `codeviz-core/src/render/mod.rs`
- `codeviz-core/src/render/mermaid.rs`
- `codeviz-core/src/lib.rs` (re-export)

## Requirements
Implement `MermaidRenderer` with three output modes selectable via `DiagramKind` enum:
1. `DiagramKind::ModuleGraph` — `graph TD` showing only `Imports` edges between `File`/`Module` nodes
2. `DiagramKind::CallGraph` — `flowchart TD` showing only `Calls` edges between `Function` nodes
3. `DiagramKind::ClassDiagram` — `classDiagram` showing `Inherits` and `Implements` edges

Rules:
- Node IDs in Mermaid output must be sanitized (replace `/`, `.`, `::` with `_`)
- If node count > 50, emit a Mermaid comment `%% WARNING: graph truncated at 50 nodes`
- Each output must be valid Mermaid syntax (tested by inspecting string structure)

## Unit Tests
Write unit tests that:
- Build a known `CodeGraph` with 3 nodes and 2 edges
- Render each `DiagramKind` and assert the output string starts with the correct Mermaid keyword
- Assert node labels are sanitized correctly
- Assert the truncation warning appears when graph exceeds 50 nodes


═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- Use stable Rust 2021 edition. Do NOT use unstable features like `let_chains`.
- Always run `cargo test --all` and `cargo clippy --all -- -D warnings` before completing the task.
