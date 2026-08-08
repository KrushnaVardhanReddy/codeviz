TASK: T31 — CFG Parser: Python + TypeScript Emitters

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Extend the Python and TypeScript parsers to emit `ControlFlowGraph` data for each
function they parse. Wire these CFGs into the `CodeGraph.control_flow` field.

Files to Modify:
- `codeviz-python/src/parser.rs` (emit CFG per function)
- `codeviz-typescript/src/parser.rs` (emit CFG per function)

Spec (READ ONLY — implement from it, never edit):
  docs/specs/ui/control_flow_graph.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- CONTEXT: Do NOT change the existing DependencyGraph output. Only ADD the `control_flow` field population.
- Use Tree-sitter node queries to detect `if_statement`, `for_statement`, `while_statement`, `try_statement`, `return_statement`, and `raise_statement` (Python) and their TypeScript equivalents.
- Map each construct to the correct `CfgBlockKind` as defined in the spec.
- No `unwrap()` in parser logic. Return `Result`.
- Ensure `cargo clippy --all -- -D warnings` and `cargo test --all` pass cleanly.

═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- Process functions in a two-pass approach:
  1. First pass: build the `DependencyGraph` nodes/edges (existing behavior).
  2. Second pass: for each function node, traverse its AST subtree to build the `ControlFlowGraph`.
- For the CFG, create an `Entry` block first, then walk the function's body statements. For each control flow statement, create the appropriate `CfgBlock` and connect it with `CfgEdge`s.
- For Python `if` statements, use tree-sitter to get the `condition` child node's source text as the `CfgBlock.label`.
- For TypeScript `async` functions, mark any `await_expression` nodes as `CfgBlockKind::AwaitPoint`.
- A simple function with no branches should produce just: `Entry → Block → Exit`.
