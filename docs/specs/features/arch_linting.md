# Spec: Architectural Linting (Phase 16)

## Overview
Z-Scan foundation. CodeViz will enforce architectural boundaries (e.g., "UI cannot import DB").

## Requirements
- Add a `[rules]` block in `codeviz.toml`.
- Syntax: `forbid = ["src/ui/** -> src/db/**"]` (meaning nodes matching the first glob cannot have an `Imports` or `Calls` edge to the second glob).
- Enhance `codeviz check` to iterate through all nodes/edges in the `CodeGraph` and assert they do not violate any rules.
- Emit clear violation strings and exit with code `1` if rules are broken.
