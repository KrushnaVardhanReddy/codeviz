# Spec: codeviz.toml Configuration Schema

## Purpose
`codeviz.toml` is the optional project-level configuration file.
CLI flags always take precedence over config file values.
Missing keys fall back to documented defaults.

---

## Full Schema
```toml
# codeviz.toml — all fields are optional

[graph]
max_depth  = 0          # 0 = unlimited edge traversal depth
max_nodes  = 50         # truncate graph above this node count
diagram_type = "module" # "module" | "call" | "class"
include    = ["**"]     # glob patterns to include (relative to source_root)
exclude    = [          # glob patterns to exclude
  "**/target/**",
  "**/node_modules/**",
  "**/.git/**",
  "**/vendor/**",
]

[languages]
enabled = ["python", "typescript", "go", "rust", "java", "kotlin"]

[output]
sentinel_start = "<!-- CODEVIZ_START -->"
sentinel_end   = "<!-- CODEVIZ_END -->"
targets = ["README.md"]   # list of markdown files to inject into

[cache]
enabled  = true
dir      = ".codeviz_cache"
```

---

## Discovery
`codeviz` searches for `codeviz.toml` by walking from the current directory upward to the filesystem root.
The first `codeviz.toml` found is used.

---

## Precedence
```
CLI flags  >  codeviz.toml  >  built-in defaults
```
Each field is independently overridable.

---

## Acceptance Criteria
- A missing `codeviz.toml` does NOT cause an error — defaults apply silently.
- A partially-filled `codeviz.toml` applies only its present keys; all others use defaults.
- Invalid TOML syntax in `codeviz.toml` prints a clear error with line number and exits 1.
- `--config /path/to/other.toml` overrides the auto-discovery search.
