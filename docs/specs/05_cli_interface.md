# Spec: CLI Interface

## Purpose
The `codeviz` CLI is the primary interface for developers and CI/CD pipelines.

---

## Commands

### `codeviz run`
Parses source code and injects an updated diagram into a markdown file.
```
codeviz run --path <dir> --output <file.md> [--diagram module|call|class] [--depth N]
```
| Flag | Default | Description |
|---|---|---|
| `--path` | `.` | Directory to scan recursively |
| `--output` | `README.md` | Markdown file with sentinel tags |
| `--diagram` | `module` | Diagram type: `module`, `call`, or `class` |
| `--depth` | unlimited | Max graph traversal depth |

Exit code: `0` on success, `1` on any error.

---

### `codeviz check`
**Read-only.** Re-generates the diagram and checks if it matches what's in the file.
Returns exit code `1` if the diagram is stale — does NOT modify the file.
```
codeviz check --path <dir> --output <file.md> [--diagram module|call|class]
```
Use in CI to gate merges without risking file modification.

---

### `codeviz watch`
Watches `--path` for file changes and re-runs automatically on save.
```
codeviz watch --path <dir> --output <file.md> [--diagram module|call|class]
```
Debounce: 300ms. Prints `[HH:MM:SS] Diagram updated.` on each refresh.
Ctrl+C to stop.

---

### `codeviz install-hook`
Auto-configures a `pre-commit` hook for the current repository.
```
codeviz install-hook [--path <dir>] [--output <file.md>]
```
- Appends the codeviz entry to `.pre-commit-config.yaml` (creates file if absent).
- Adds sentinel tags to `--output` if they are missing.
- Prints what it changed.

---

### `codeviz export`
Exports the CodeGraph in an alternative format.
```
codeviz export --path <dir> --format json|dot [--output <file>]
```
If `--output` is omitted, writes to stdout.

---

### `codeviz serve --mcp`
Starts an MCP server over stdio (JSON-RPC 2.0).
```
codeviz serve --mcp [--port N]
```
If `--port` is given, uses HTTP/SSE transport instead of stdio.

---

## Global Flags
| Flag | Description |
|---|---|
| `--config <path>` | Path to `codeviz.toml` (default: auto-discover from cwd upward) |
| `--verbose` | Print per-file parse details |
| `--quiet` | Suppress all output except errors |
| `--version` | Print version and exit |

---

## Acceptance Criteria
- `codeviz --help` prints usage without error.
- `codeviz run` with valid args exits 0 and modifies the output file.
- `codeviz check` with an up-to-date diagram exits 0 without modifying the file.
- `codeviz check` with a stale diagram exits 1 without modifying the file.
- Unknown subcommand prints help and exits 1.
