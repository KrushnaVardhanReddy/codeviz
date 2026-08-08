# Spec: Control Flow Graph (Phase 12)

## Overview
Phase 12 extends the `CodeGraph` IR with a second graph type — the **Control Flow
Graph (CFG)** — which captures intra-function control flow constructs (conditions,
loops, error handling). The CFG is rendered in the Web UI when a user drills into a
specific function node.

---

## Architecture Decision: Two Linked Graph Types

The `CodeGraph` IR contains two logically separate graphs:

1. **`DependencyGraph`** (Phase 1-10): Module/class/function level. Architecture overview.
2. **`ControlFlowGraph`** (Phase 12): Function-internal level. Logic flow per function.

Each `Function` node in the `DependencyGraph` may optionally contain a `ControlFlowGraph`
for that function. They are linked by the function's node `id`.

---

## New IR Structs (additions to `codeviz-core/src/ir.rs`)

```rust
/// A control flow graph for a single function.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ControlFlowGraph {
    /// The ID of the function node this CFG belongs to.
    pub function_id: String,
    /// All basic blocks / control flow nodes.
    pub blocks: Vec<CfgBlock>,
    /// Edges between blocks.
    pub cfg_edges: Vec<CfgEdge>,
}

/// A single block in the control flow graph.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CfgBlock {
    pub id: String,
    pub kind: CfgBlockKind,
    /// Human-readable label (e.g., the condition expression)
    pub label: String,
    /// 1-indexed line number in source
    pub line: Option<u32>,
}

/// The kind of a CFG block.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CfgBlockKind {
    /// Function entry point
    Entry,
    /// Function exit / return
    Exit,
    /// A plain statement block
    Block,
    /// An if/else condition (decision diamond)
    Condition,
    /// A loop header (for/while/do-while)
    LoopHeader,
    /// A loop body
    LoopBody,
    /// A match/switch arm
    SwitchArm,
    /// A try block
    TryBlock,
    /// A catch/except block
    CatchBlock,
    /// A finally block
    FinallyBlock,
    /// An async/await suspension point
    AwaitPoint,
    /// A throw/raise error propagation
    ThrowPoint,
}

/// An edge between CFG blocks.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CfgEdge {
    pub from_id: String,
    pub to_id: String,
    pub kind: CfgEdgeKind,
    /// Optional label (e.g., "true", "false", "catch TypeError")
    pub label: Option<String>,
}

/// The kind of a CFG edge.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CfgEdgeKind {
    /// Normal sequential flow
    Normal,
    /// True branch of an if/condition
    TrueBranch,
    /// False branch of an if/condition
    FalseBranch,
    /// Loop back edge (creates cycle)
    LoopBack,
    /// Exception propagation to catch block
    ExceptionEdge,
    /// Always-runs path (finally block)
    FinallyEdge,
    /// Async suspension / resume
    AsyncEdge,
}
```

---

## Visual Design for CFG (in the React Flow Side Panel)

| Block Kind | Color | Shape |
|---|---|---|
| `Entry` | 🟢 Green circle | Oval |
| `Exit` | 🔴 Red circle | Oval |
| `Block` | ⬜ White rect | Rectangle |
| `Condition` | 🔵 Blue diamond | Diamond (rotated square) |
| `LoopHeader` | 🟡 Yellow diamond | Diamond with loop icon |
| `TryBlock` | 🟠 Orange rect | Rectangle |
| `CatchBlock` | 🔴 Red rect | Rectangle |
| `FinallyBlock` | 🟣 Purple rect | Rectangle |
| `AwaitPoint` | 🟣 Purple pill | Pill shape |
| `ThrowPoint` | 🔴 Red octagon | Octagon / stop shape |

| Edge Kind | Color | Style |
|---|---|---|
| `Normal` | ⚫ Black | Solid arrow |
| `TrueBranch` | 🟢 Green | Solid, labeled "✓ true" |
| `FalseBranch` | 🔴 Red | Solid, labeled "✗ false" |
| `LoopBack` | 🟡 Yellow | Curved back arrow |
| `ExceptionEdge` | 🔴 Red | Dashed arrow |
| `FinallyEdge` | 🟣 Purple | Dotted arrow |
| `AsyncEdge` | 🟣 Purple | Wavy arrow |

---

## Example: Python Function

Source:
```python
async def fetch_user(user_id: str):
    try:
        if user_id is None:
            raise ValueError("ID required")
        user = await db.get(user_id)
        return user
    except Exception as e:
        log_error(e)
```

Expected CFG:
```
[Entry]
   │ Normal
[Condition: user_id is None]
   │ TrueBranch (✓)         │ FalseBranch (✗)
[ThrowPoint: ValueError]  [AwaitPoint: db.get()]
                              │ AsyncEdge
                          [Block: return user]
                              │ Normal
                          [Exit]
       │ ExceptionEdge (from try block)
[CatchBlock: Exception as e]
       │ Normal
[Block: log_error(e)]
       │ Normal
[Exit]
```

---

## Acceptance Criteria
- [ ] New `ControlFlowGraph` struct added to `codeviz-core/src/ir.rs`.
- [ ] `CfgBlock`, `CfgEdge`, `CfgBlockKind`, `CfgEdgeKind` all have full serde support.
- [ ] Python parser (Task 06 extension) emits CFGs for all functions.
- [ ] TypeScript parser (Task 09 extension) emits CFGs for all functions.
- [ ] Web UI side panel renders the CFG using the visual design system above.
- [ ] All new IR types have unit tests.
