# Spec: Check Mode (`codeviz check`)

## Purpose
A **read-only** CI validation command. Re-generates the diagram and compares it
to what's already in the output file. Never writes to disk.

---

## Behavior
1. Parse `--path` using the same pipeline as `codeviz run`.
2. Render the Mermaid diagram.
3. Read the current diagram from `--output` (between sentinel tags).
4. Compare the two strings (whitespace-normalized).
5. If identical → exit 0, print `✅ Diagram is up-to-date.`
6. If different → exit 1, print a unified diff of old vs new, print `❌ Diagram is stale. Run codeviz run to update.`

---

## Whitespace Normalization
Before comparing, normalize both strings:
- Trim leading/trailing whitespace per line
- Collapse multiple blank lines into one
- Ignore trailing newline differences

---

## CLI
```
codeviz check --path <dir> --output <file.md> [--diagram module|call|class]
```

---

## Acceptance Criteria
- `codeviz check` on a freshly `run` file exits 0.
- `codeviz check` after modifying source code without re-running exits 1.
- The output file is NOT modified under any circumstances.
- The exit code is usable in `if codeviz check; then ...` shell scripts.
