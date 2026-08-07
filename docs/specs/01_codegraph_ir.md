# Spec: CodeGraph IR (Intermediate Representation)

## Purpose
The `CodeGraph` is the **central data contract** of CodeViz. Every language parser produces one.
Every renderer and adapter consumes one. The IR must remain language-agnostic.

---

## Data Types

```rust
pub struct CodeGraph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub meta:  GraphMeta,
}

pub struct Node {
    pub id:        String,       // globally unique: "{file_path}::{symbol_name}"
    pub label:     String,       // display name, stripped of generics/lifetimes
    pub kind:      NodeKind,
    pub file_path: String,       // relative to source_root
    pub line:      Option<u32>,  // 1-indexed line number; None if not resolvable
    pub is_public: bool,         // true if exported/pub
}

pub enum NodeKind {
    File,
    Module,
    Function { is_async: bool },
    Class,
    Interface,
    Constant,
}

pub struct Edge {
    pub from_id: String,  // must match a Node.id
    pub to_id:   String,  // must match a Node.id, or be an unresolved external
    pub kind:    EdgeKind,
}

pub enum EdgeKind {
    Imports,       // module-level dependency
    Calls,         // function invokes function
    Inherits,      // class extends class / trait extends trait
    Implements,    // class implements interface / impl Trait for Struct
    Returns,       // function returns type
    Instantiates,  // function creates instance of class
}

pub struct GraphMeta {
    pub language:     String,  // e.g. "python", "typescript"
    pub source_root:  String,  // absolute path scanned
    pub generated_at: String,  // ISO 8601 UTC timestamp
    pub node_count:   usize,
    pub edge_count:   usize,
}
```

---

## Constraints
- `Node.id` must be unique within a `CodeGraph`. Parsers must enforce this.
- `Edge.from_id` must reference an existing `Node.id` in the same graph.
- `Edge.to_id` may reference an external/unresolved node (use the raw import path as id).
- Circular edges are allowed — renderers handle display.
- `GraphMeta.node_count` and `edge_count` must equal `nodes.len()` and `edges.len()`.

---

## Serialization
All types must implement `serde::Serialize` and `serde::Deserialize`.
The serialized JSON format is the stable public API for MCP tool responses and `codeviz export --format json`.

---

## Acceptance Criteria
- Round-trip: `serde_json::to_string(&graph) → serde_json::from_str()` produces identical struct.
- A `CodeGraph` with 0 nodes and 0 edges is valid (empty file or no symbols found).
- `NodeKind::Function { is_async: true }` serializes as `{"Function": {"is_async": true}}`.
