TASK: T55 — MCP Debugging Tools & Python Call Parsing

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement the MCP debugging tools (`trace_call_path`, `get_callers_recursive`, and `get_blast_radius`) in the `codeviz-mcp` crate. Because these backend tools rely heavily on `EdgeKind::Calls` and `EdgeKind::Contains`, you must first upgrade the `codeviz-python` parser to correctly extract hierarchical relationships (`parent_id`) and perform scope-aware function call resolution.

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/blast_radius.md
  docs/specs/features/hierarchical_drilldown.md

═══════════════════════════════════════════════════════════════
BACKGROUND — WHAT ALREADY EXISTS (READ ONLY)
═══════════════════════════════════════════════════════════════

codeviz-python/src/parser.rs
  - Currently extracts Classes, Functions, and Imports.
  - Sets `parent_id = None` universally on all nodes (which flattens the graph).
  - DOES NOT extract function calls at all.

codeviz-mcp/src/server.rs
  - Implements basic MCP server functionality.
  - Contains stubs for `get_blast_radius` and others, but they need to do BFS traversal over the `CodeGraph`.

═══════════════════════════════════════════════════════════════
DELIVERABLES
═══════════════════════════════════════════════════════════════

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. MODIFY: codeviz-python/src/parser.rs (Hierarchical Nesting)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
- Update `extract_class` and `extract_function` to accept and track scope context.
- If a function is inside a class, set its `parent_id` to the class's `id`.
- If a class/function is defined at the file root, set its `parent_id` to the file's `id`.
- Automatically push `EdgeKind::Contains` edges from the parent to the child to formally link them in the graph for downstream layout engines.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
2. MODIFY: codeviz-python/src/parser.rs (Scope-Based Call Resolution)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
- Walk the AST of function bodies to find `call` nodes (tree-sitter node type: `"call"`).
- Maintain a local scope map of imports (e.g., `from X import Y`) and local declarations while walking the file.
- When resolving a call's target:
  - If the call matches an import (e.g., `Y()`), emit `EdgeKind::Calls` pointing exactly to `X::Y`.
  - If the call is a method invocation (e.g., `self.foo()`), emit `EdgeKind::Calls` pointing to `<CurrentClass>::foo`.
  - If the call matches a locally defined function `bar()`, emit pointing to `<CurrentFile>::bar`.
  - Otherwise, emit to `<called_name>` (as an External call).

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
3. MODIFY: codeviz-mcp/src/server.rs (MCP Tools)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
- **get_blast_radius**: Perform a backward BFS from the target node, following inbound `EdgeKind::Calls` and `EdgeKind::Imports` edges, up to a configurable max depth. Return the resulting subgraph.
- **get_callers_recursive**: Similar to blast radius, but strictly returns linear call chains (callers of callers).
- **trace_call_path**: Finds the shortest path (Dijkstra/BFS) between a `source_node` and `target_node` using `EdgeKind::Calls`.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
4. ADD TESTS: codeviz-python/src/parser.rs & codeviz-mcp/src/server.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
- Add unit tests verifying `parent_id` assignment for nested classes/methods.
- Add unit tests verifying scope-aware call resolution (`self.method()`, `imported_func()`).
- Add tests validating that the BFS logic in the MCP endpoints does not infinitely loop on recursive calls.

═══════════════════════════════════════════════════════════════
EXECUTION / VERIFICATION
═══════════════════════════════════════════════════════════════
- Build and test `codeviz-python` using `cargo test -p codeviz-python`.
- Build and test `codeviz-mcp` using `cargo test -p codeviz-mcp`.
- Ensure there are no compiler warnings.

═══════════════════════════════════════════════════════════════
SAFEGUARDS
═══════════════════════════════════════════════════════════════
- DO NOT rewrite the entire parser file; surgically insert the scope map and AST block walking logic.
- Ensure that the scope map perfectly normalizes paths to match the existing graph `id` formatting (e.g. forward slashes for paths).
