# Spec: Mermaid Renderer

## Purpose
The `MermaidRenderer` converts a `CodeGraph` into a valid Mermaid diagram string.
It is the only component allowed to produce Mermaid output — no other adapter should generate Mermaid directly.

---

## Diagram Kinds
```rust
pub enum DiagramKind {
    ModuleGraph,   // graph TD  — Imports edges only
    CallGraph,     // flowchart TD — Calls edges only
    ClassDiagram,  // classDiagram — Inherits + Implements edges
}
```

---

## Output Format

### ModuleGraph
```
graph TD
    src_main --> src_config
    src_main --> src_parser
```

### CallGraph
```
flowchart TD
    parse_file --> extract_nodes
    extract_nodes --> build_edge
```

### ClassDiagram
```
classDiagram
    Animal <|-- Dog
    IRunnable <|.. Dog
```

---

## Node ID Sanitization
Node IDs in Mermaid must not contain `/`, `.`, `::`, `-`, or spaces.
Replace all such characters with `_`.

Example: `src/parser.rs::parse_file` → `src_parser_rs__parse_file`

---

## Truncation
If filtered node count > `max_nodes` (default 50), truncate and prepend:
```
%% WARNING: graph truncated — showing 50 of N nodes
```

---

## Renderer API
```rust
pub struct MermaidRenderer {
    pub max_nodes: usize,  // default: 50
}

impl MermaidRenderer {
    pub fn render(&self, graph: &CodeGraph, kind: DiagramKind) -> String;
}
```

---

## Acceptance Criteria
- Output for `ModuleGraph` always starts with `graph TD`.
- Output for `CallGraph` always starts with `flowchart TD`.
- Output for `ClassDiagram` always starts with `classDiagram`.
- An empty graph (0 nodes) produces valid but empty Mermaid (just the header line).
- Node IDs never contain `/`, `.`, `::`, `-`, or spaces.
- Truncation warning appears when node count exceeds `max_nodes`.
