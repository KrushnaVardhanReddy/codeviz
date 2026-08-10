TASK: T36 — Git History & Hotspots Extraction

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Enhance the CodeGraph by integrating with the local Git repository to extract
file churn (how often a file changes) and authorship. This data will be attached
to `NodeMeta` to allow the UI to visualize behavioral hotspots.

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/git_history.md

═══════════════════════════════════════════════════════════════
BACKGROUND — WHAT ALREADY EXISTS
═══════════════════════════════════════════════════════════════

- The `CodeGraph` structure is in `codeviz-core/src/graph.rs`.
- The CLI orchestrates parsing in `codeviz-cli/src/main.rs`.

═══════════════════════════════════════════════════════════════
DELIVERABLES
═══════════════════════════════════════════════════════════════

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. MODIFY: codeviz-core/Cargo.toml
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Add the `gix` (gitoxide) crate for fast, pure-Rust Git traversal.
Add `rayon` if not already present for parallelization.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
2. CREATE: codeviz-core/src/git.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Implement a fast Git history extractor.
- For each file in the repository, calculate `churn_score` (number of commits modifying it in the last 6 months).
- Calculate `primary_authors` (top 3 authors by commits for that file).
- This process must be highly parallelized.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
3. MODIFY: codeviz-core/src/graph.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Update `NodeMeta` (or `Node`) to include `churn_score` (u32) and `primary_authors` (Vec<String>).
Update the JSON serialization to emit these fields.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
4. MODIFY: CLI Orchestration
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
In `codeviz-cli/src/main.rs`, after generating the base `CodeGraph` via the language parsers,
run the Git history extraction concurrently or sequentially, and merge the data into the graph nodes.

═══════════════════════════════════════════════════════════════
CRITICAL IMPLEMENTATION RULES
═══════════════════════════════════════════════════════════════
1. Do NOT block AST parsing. Git extraction should be fast.
2. Gracefully handle non-Git repositories (fallback to 0 churn).
3. Ensure `cargo test --all` and `cargo clippy --all -- -D warnings` pass.

Commit: "jules: T36 — Git History & Hotspots Extraction"
Target branch: feat-t36-git-history
