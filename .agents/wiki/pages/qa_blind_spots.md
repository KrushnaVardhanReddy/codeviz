---
title: QA & Blind Spots Mitigation
tags: [qa, windows, vscode, performance, e2e]
source_count: 1
---

# QA & Blind Spots Mitigation

This concept covers the strategies used to mitigate testing and performance blind spots in CodeViz.

## Key Concepts
- **Cross-Platform Path Normalization**: The Rust core engine aggressively normalizes all file paths to use forward slashes (`/`), even on Windows (`\`), to ensure consistent graph IDs and module resolution across platforms.
- **VS Code Extension Testing**: Playwright is insufficient for testing native editor extensions. We use `@vscode/test-electron` to validate the `codeviz-vscode` extension inside a headless VS Code instance.
- **Graph Stress Testing**: To prevent browser crashes on massive enterprise repositories, we enforce Playwright performance assertions against synthetic fixtures containing >10,000 nodes and 20,000 edges.
