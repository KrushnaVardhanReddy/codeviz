# Spec: QA & Blind Spots Mitigation

## Overview
This specification addresses three major QA blind spots identified in the MVP testing strategy: Cross-Platform Path Normalization, VS Code Extension E2E Testing, and Graph Rendering Stress Tests.

## Requirements

### 1. Cross-Platform Path Normalization (T60)
- **Problem**: The Rust CLI parses file paths using native OS separators. On Windows, paths use `\`, which breaks module resolution, web imports, and graph IDs (which expect `/`).
- **Solution**: 
  - Ensure all internal paths in the `ir::CodeGraph` (specifically `Node::id` and `Node::file_path`) are normalized to use forward slashes (`/`).
  - Add a Windows CI runner (`windows-latest`) for Playwright tests in `.github/workflows/ci.yml`.

### 2. VS Code Extension E2E Testing (T61)
- **Problem**: Playwright only tests the Next.js web application, leaving the `codeviz-vscode` extension untested in CI.
- **Solution**:
  - Integrate `@vscode/test-electron` to spin up a headless VS Code instance.
  - Assert that the extension activates on a workspace with a `codeviz.toml` file, opens the CodeViz Explorer panel, and renders the Mermaid graph in the Webview.

### 3. Graph Rendering Stress Test (T62)
- **Problem**: React Flow can suffer severe performance degradation or crash the browser when rendering large codebases (>10,000 nodes/edges).
- **Solution**:
  - Implement a stress testing script that generates a synthetic `CodeGraph` JSON fixture with 10,000 nodes and 20,000 edges.
  - Add Playwright performance assertions to measure time-to-interactive (TTI) for the graph canvas, ensuring it remains responsive.
  - (If it fails, this will trigger the need for canvas virtualization in a future task).
