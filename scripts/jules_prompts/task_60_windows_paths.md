# TASK: T60 — Cross-Platform Path Normalization

Please implement cross-platform path normalization.

1. **Read Spec**: `docs/specs/features/qa_blind_spots.md`
2. **Context**: The Rust CLI breaks on Windows because it parses `\` instead of `/` for paths.
3. **Execution**:
   - Update `codeviz-core/src/graph.rs` and the parsers to ensure that `Node::id` and `Node::file_path` always use forward slashes (`/`), even on Windows.
   - Add a `windows-latest` runner for the Playwright E2E tests in `.github/workflows/ci.yml`.
   - Ensure `cargo test --all` passes.
