# Spec: Code Coverage Overlay (Phase 18)

## Overview
Test coverage is a crucial metric, but it is often viewed in isolated HTML reports.
CodeViz will allow users to overlay standard coverage data (e.g., LCOV format) onto the `CodeGraph`. This highlights visually which critical modules are lacking tests.

## Coverage Input
CodeViz will accept coverage data via a new CLI argument `--coverage-file <path>`.
We will parse a standard coverage format. **LCOV** is the most widely supported common denominator (used by `lcov` in C/C++, `istanbul`/`nyc` in JS/TS, `grcov` for Rust).

## Graph Augmentation
1. Parse the LCOV file to extract file paths and their line coverage percentage.
2. For every `NodeKind::File` in the `CodeGraph`, look up its path in the coverage data.
3. Augment the `Node` with a new optional field: `coverage_percent: Option<f64>`.

## CLI Output
When generating a Mermaid diagram (`codeviz run --output mermaid --coverage-file lcov.info`):
- Assign colors to nodes based on coverage (e.g., Green > 80%, Yellow 50-80%, Red < 50%, Gray for unknown).
- Use Mermaid's `style` directives to color the nodes.

## MCP Server Integration
Update the `get_module_graph` tool to optionally accept a `coverage_file` parameter, and if provided, the returned JSON `CodeGraph` should include the `coverage_percent` on nodes.
