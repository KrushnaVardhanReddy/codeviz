TASK: T39 — Architectural Linting (Z-Scan)

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement architectural boundary enforcement via a `[rules]` block in `codeviz.toml`.
When `codeviz check` is run, it must evaluate all edges in the `CodeGraph` against
the defined rules and exit with code `1` if any are violated.

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/arch_linting.md

═══════════════════════════════════════════════════════════════
BACKGROUND — WHAT ALREADY EXISTS
═══════════════════════════════════════════════════════════════

- Config file parsing exists in the codebase for `codeviz.toml`.
- The `check` subcommand exists in `codeviz-cli/src/main.rs`.
- `CodeGraph` edges are typed as `Imports` or `Calls`.

═══════════════════════════════════════════════════════════════
DELIVERABLES
═══════════════════════════════════════════════════════════════

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. MODIFY: Config parsing (codeviz.toml)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Add support for the following new block in `codeviz.toml`:
```toml
[rules]
forbid = [
  "src/ui/** -> src/db/**",
  "src/api/** -> src/ui/**"
]
```
Each rule string is parsed into a `(source_glob, target_glob)` pair.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
2. CREATE: codeviz-core/src/linter.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Implement a `lint(graph: &CodeGraph, rules: &[Rule]) -> Vec<LintViolation>` function.
- For each edge in the graph, check if the source node's `file_path` matches the
  source glob AND the target node's `file_path` matches the target glob.
- If both match, add a `LintViolation` to the result.
- Use the `glob` crate for glob matching.

Use this struct:
```rust
pub struct LintViolation {
    pub rule: String,
    pub source_node: String,
    pub target_node: String,
}
```

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
3. MODIFY: codeviz-cli/src/main.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
In the `check` subcommand, after generating the `CodeGraph`, call the linter.
- If violations exist, print each one clearly (e.g., `[RULE VIOLATION] ui/Login.tsx -> db/query.rs violates: "src/ui/** -> src/db/**"`)
- Exit with code `1`.
- If no violations, print `✅ All architectural rules passed.` and exit with `0`.

═══════════════════════════════════════════════════════════════
CRITICAL IMPLEMENTATION RULES
═══════════════════════════════════════════════════════════════
1. Use the `glob` crate for matching. Do NOT implement custom glob logic.
2. Ensure `cargo test --all` and `cargo clippy --all -- -D warnings` pass.
3. Add unit tests verifying glob matching logic.

Commit: "jules: T39 — Architectural linting (Z-Scan rules engine)"
Target branch: feat-t39-arch-linting
