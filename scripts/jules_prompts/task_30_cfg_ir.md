TASK: T30 — CFG IR Extension in codeviz-core

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Extend the `CodeGraph` IR in `codeviz-core` with the full Control Flow Graph types
as defined in the spec. This task is purely the IR layer — no parser changes yet.

Files to Modify/Create:
- `codeviz-core/src/ir.rs` (add ControlFlowGraph, CfgBlock, CfgEdge, CfgBlockKind, CfgEdgeKind)
- `codeviz-core/src/lib.rs` (re-export new types)

Spec (READ ONLY — implement from it, never edit):
  docs/specs/ui/control_flow_graph.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- CONTEXT: Add the new structs to `ir.rs` alongside the existing `CodeGraph` types. Do NOT change the existing `CodeGraph`, `Node`, `Edge`, `NodeKind`, or `EdgeKind` structs — only ADD new types.
- All new types must derive `Debug, Clone, Serialize, Deserialize, PartialEq`.
- All new public types and fields must have `///` doc comments.
- No `unwrap()` anywhere.
- Write unit tests that construct a simple `ControlFlowGraph` and assert its fields.
- Ensure `cargo clippy --all -- -D warnings` and `cargo test --all` pass cleanly.

═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- The new `ControlFlowGraph` struct should live in `ir.rs` right below the existing `GraphMeta` struct.
- Add a `control_flow: Option<Vec<ControlFlowGraph>>` field to the existing `CodeGraph` struct. Use `Option` so that codebases parsed without CFG support still have a valid `CodeGraph` with `control_flow: None`.
- Use `#[serde(skip_serializing_if = "Option::is_none")]` on that field so old JSON exports remain clean.
- The simplest unit test is to build a CFG for a two-block function: an `Entry` block connected by a `Normal` edge to an `Exit` block.
