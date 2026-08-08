TASK: T36 — Git History Integration

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Integrate with the local Git repository to extract file churn (how often a file changes) and authorship (who owns a file), attaching this data to the `CodeGraph` nodes.

Files to Modify/Create:
- `codeviz-core/src/git.rs` (new module for git operations)
- `codeviz-core/src/ir.rs` (add churn and author fields to node meta)
- `codeviz-cli/src/main.rs` (trigger git extraction)

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/git_history.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- Use the `gix` (gitoxide) crate. Do not use `git2` if possible due to C dependencies.
- Git traversal must be fast and parallelized using `rayon`.
- Do not panic on non-git repos; degrade gracefully.

═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- Add `churn_score: u32` and `primary_authors: Vec<String>` to `NodeMeta` in `ir.rs`.
- Run the git extraction concurrently with the AST parsing if possible, or right after.
