# Jules Task 02 — CodeGraph IR Structs

## Objective
Define the language-agnostic `CodeGraph` Intermediate Representation (IR) in `codeviz-core`.
This is the core data model — every language parser outputs a `CodeGraph`, every renderer consumes one.

## Files to Modify/Create
- `codeviz-core/src/graph.rs`
- `codeviz-core/src/lib.rs` (re-export `graph` module)
- `codeviz-core/Cargo.toml` (add `serde` + `serde_json` dependencies)

## Requirements
Define the following types with full `serde::Serialize/Deserialize` derives:

```
CodeGraph { nodes: Vec<Node>, edges: Vec<Edge>, meta: GraphMeta }

Node {
    id: String,          // unique, e.g. "src/parser.rs::parse_file"
    label: String,       // display name
    kind: NodeKind,
    file_path: String,
    line: Option<u32>,
}

NodeKind (enum): File | Module | Function { is_async: bool } | Class | Interface | Constant

Edge {
    from_id: String,
    to_id: String,
    kind: EdgeKind,
}

EdgeKind (enum): Imports | Calls | Inherits | Implements | Returns | Instantiates

GraphMeta {
    language: String,
    source_root: String,
    generated_at: String,  // ISO 8601 timestamp
}
```

## Unit Tests
Add unit tests in `codeviz-core/src/graph.rs` that:
- Serialize and deserialize a minimal `CodeGraph` round-trip via `serde_json`
- Assert all field values survive the round-trip unchanged


═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- Use stable Rust 2021 edition. Do NOT use unstable features like `let_chains`.
- Always run `cargo test --all` and `cargo clippy --all -- -D warnings` before completing the task.
