# WASM Basic Call Edges Heuristic

## Overview
During MVP v1 testing, it was discovered that the `codeviz-wasm` playground AST parser generated nodes for functions and classes, but completely omitted the extraction of `Edge` structs to connect them. This resulted in floating, disconnected nodes in the React Flow playground canvas.

## Implementation Details
A lightweight heuristic was added directly into `extract_from_ast` inside `codeviz-wasm/src/lib.rs`.

1. **Scope Tracking**:
   The recursive `extract_from_ast` function now takes an `Option<&str>` for `current_scope_id`.
   When the parser encounters a `function_definition` (or class), it resolves the `id` (e.g. `main.py::step_one`) and passes this down to children as the new `current_scope_id`.

2. **Edge Extraction**:
   If a `call` (Python) or `call_expression` (TypeScript) node is encountered, the parser checks if there is an active `current_scope_id`.
   If so, it searches the immediate children for an `identifier` or `property_identifier`.
   It constructs the target node ID (e.g. `main.py::target_name`) and pushes a new `Edge` of kind `EdgeKind::Calls` to `graph.edges`.

## Limitations
This heuristic is highly simplistic. It works for flat, procedural function calls (e.g., `step_one(val)`), but it completely misses method calls wrapped in attribute nodes (e.g., `d.bark()`). It also does not extract inheritance structures.
