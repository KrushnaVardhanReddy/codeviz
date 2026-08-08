# Spec: Git History & Hotspots (Phase 16)

## Overview
To provide deeper behavioral insights (similar to CodeScene), CodeViz will integrate
with the local Git repository to extract file churn (how often a file changes) and
authorship (who owns a file). This data will be attached to the `CodeGraph` nodes.

## Requirements
- Use the `gix` (gitoxide) or `git2` crate to read the local repository history.
- For each file in the `CodeGraph`, calculate:
  - `churn_score`: Total number of commits that modified this file in the last 6 months.
  - `primary_authors`: Top 3 authors by lines changed or commits.
- Add these fields to `NodeMeta` (or directly on `Node`) in the `CodeGraph` IR.

## Performance Constraints
- Git traversal must be fast and heavily parallelized (using Rayon).
- Do not block AST parsing; run Git history extraction concurrently.
