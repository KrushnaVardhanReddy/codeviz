# Spec: Multiple Output Targets

## Purpose
Allow CodeViz to update multiple markdown files in a single run —
e.g., a root `README.md` for the module graph and a `docs/ARCHITECTURE.md` for the class diagram.

---

## Configuration
```toml
# codeviz.toml
[[outputs]]
file         = "README.md"
diagram_type = "module"

[[outputs]]
file         = "docs/ARCHITECTURE.md"
diagram_type = "class"

[[outputs]]
file         = "docs/CALLGRAPH.md"
diagram_type = "call"
```

Each `[[outputs]]` entry is processed independently.

---

## CLI Override
`--output` still works for a single target:
```bash
codeviz run --path ./src --output README.md
```
When `--output` is supplied on CLI, it overrides the `[[outputs]]` list entirely.

---

## Behavior
- Parse the source tree **once** (shared `CodeGraph`).
- For each output entry: render the requested diagram type, inject into the target file.
- Print one status line per output file.

---

## Acceptance Criteria
- Three output files are all updated in a single `codeviz run` invocation.
- Each file gets the correct diagram type.
- A single output file with a missing sentinel tag logs an error and continues to the next file (does not abort the whole run).
- The source tree is parsed only once, not once per output target.
