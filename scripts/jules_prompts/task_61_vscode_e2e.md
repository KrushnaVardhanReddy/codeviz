# TASK: T61 — VS Code Extension E2E Tests

Please implement native E2E tests for the VS Code Extension.

1. **Read Spec**: `docs/specs/features/qa_blind_spots.md`
2. **Context**: We need to test `codeviz-vscode` within a real headless VS Code environment.
3. **Execution**:
   - Add `@vscode/test-electron` to `codeviz-vscode/package.json`.
   - Write an integration test in `codeviz-vscode/src/test/suite/extension.test.ts` that activates the extension.
   - Assert that the "CodeViz Explorer" panel opens and successfully runs the CodeViz CLI binary.
   - Run `npm test` inside `codeviz-vscode` to verify it passes.
