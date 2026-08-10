TASK: T61 — VS Code Extension E2E Tests

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Add real integration tests to the `codeviz-vscode` extension using the
`@vscode/test-electron` framework. The tests must spin up a headless VS Code
instance, load the extension in a real workspace, and assert that the
"CodeViz Explorer" panel activates and renders a Mermaid graph.

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/qa_blind_spots.md (Section 2: VS Code Extension E2E Testing)

═══════════════════════════════════════════════════════════════
BACKGROUND — WHAT ALREADY EXISTS (READ ONLY — do not modify unless specified)
═══════════════════════════════════════════════════════════════

EXTENSION SOURCE (READ — understand before writing tests):
  codeviz-vscode/src/extension.ts
    - activate() is called when the extension starts.
    - It checks for the existence of `codeviz.toml` in the workspace root.
    - If the file exists, it registers `GraphPanelProvider` as a WebviewViewProvider
      under the viewType `"codeviz-explorer"`.
    - It registers commands: `codeviz.showGraph`, `codeviz.refreshGraph`,
      `codeviz.openWebUi`, `codeviz.showOutput`.
    - It subscribes to `onDidSaveTextDocument` and `onDidChangeActiveTextEditor`
      to auto-refresh the graph.

  codeviz-vscode/src/graphPanel.ts
    - Implements `vscode.WebviewViewProvider`.
    - Static property: `GraphPanelProvider.viewType = "codeviz-explorer"`.
    - `updateGraph(mermaidStr: string)` sends a postMessage to the webview.

  codeviz-vscode/src/statusBar.ts
    - `CodeVizStatusBar` with methods: `setParsing()`, `setReady()`, `setError()`.

EXISTING TEST INFRASTRUCTURE:
  codeviz-vscode/package.json
    - "test" script: `node ./out/test/runTest.js`
    - "@vscode/test-electron" is already listed as a devDependency.
  codeviz-vscode/src/test/runTest.ts
    - Sets up the test runner pointing to `./out/test/suite/index`.
  codeviz-vscode/src/test/suite/index.ts
    - Mocha runner configuration.
  codeviz-vscode/src/test/suite/extension.test.ts  (MODIFY — replace/extend)
    - Currently contains only a trivial "Sample test" that asserts array indexOf.
    - You will REPLACE this with real extension integration tests.

BUILD:
  `npm run compile` compiles TypeScript with `tsc -p ./`.
  `npm test` compiles and runs `node ./out/test/runTest.js`.

═══════════════════════════════════════════════════════════════
DELIVERABLES
═══════════════════════════════════════════════════════════════

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. CREATE: A temporary fixture workspace for tests
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Create the following fixture files (committed to the repo):

  codeviz-vscode/test-fixtures/workspace/codeviz.toml
    Contents:
      [project]
      name = "test-fixture"

  codeviz-vscode/test-fixtures/workspace/hello.py
    Contents:
      def hello():
          pass

      def world():
          hello()

These fixture files represent a minimal workspace that will activate the extension.


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
2. MODIFY: codeviz-vscode/src/test/runTest.ts
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Update runTest.ts to open the fixture workspace when launching the test VS Code
instance. The `extensionTestsPath` and `launchArgs` must be set correctly:

  import * as path from 'path';
  import { runTests } from '@vscode/test-electron';

  async function main() {
    const extensionDevelopmentPath = path.resolve(__dirname, '../../');
    const extensionTestsPath = path.resolve(__dirname, './suite/index');
    // Open the fixture workspace so the extension activates
    const workspacePath = path.resolve(__dirname, '../../test-fixtures/workspace');

    await runTests({
      extensionDevelopmentPath,
      extensionTestsPath,
      launchArgs: [workspacePath],
    });
  }

  main().catch(err => {
    console.error('Failed to run tests', err);
    process.exit(1);
  });


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
3. MODIFY: codeviz-vscode/src/test/suite/extension.test.ts
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Replace the trivial sample test with FOUR real integration tests:

  import * as assert from 'assert';
  import * as vscode from 'vscode';
  import * as path from 'path';
  import * as fs from 'fs';

  suite('CodeViz Extension Test Suite', () => {

    // --- Test 1: Extension activates successfully ---
    test('extension should activate when codeviz.toml is present', async () => {
      // The fixture workspace already has codeviz.toml, so the extension
      // should have activated automatically when VS Code opened.
      const ext = vscode.extensions.getExtension('codeviz.codeviz');
      assert.ok(ext, 'Extension should be found');
      await ext!.activate();
      assert.strictEqual(ext!.isActive, true, 'Extension should be active');
    });

    // --- Test 2: Commands are registered ---
    test('should register all CodeViz commands', async () => {
      const commands = await vscode.commands.getCommands(true);
      assert.ok(commands.includes('codeviz.showGraph'), 'codeviz.showGraph must be registered');
      assert.ok(commands.includes('codeviz.refreshGraph'), 'codeviz.refreshGraph must be registered');
      assert.ok(commands.includes('codeviz.openWebUi'), 'codeviz.openWebUi must be registered');
    });

    // --- Test 3: Executing showGraph command does not throw ---
    test('executing codeviz.showGraph should not throw', async () => {
      // This command focuses the sidebar panel. It should complete without error.
      await assert.doesNotReject(
        vscode.commands.executeCommand('codeviz.showGraph'),
        'codeviz.showGraph command should not throw'
      );
    });

    // --- Test 4: Extension does NOT activate without codeviz.toml ---
    test('extension should skip activation if codeviz.toml is missing', async () => {
      // Open a temporary workspace WITHOUT a codeviz.toml file.
      // The activate() function returns early if the config file is missing.
      // We test this by directly importing and calling the activate() function
      // with a mock context pointing to a temp directory.
      const tmpDir = fs.mkdtempSync(path.join(require('os').tmpdir(), 'codeviz-test-'));
      try {
        // Ensure no codeviz.toml exists in tmpDir
        assert.ok(!fs.existsSync(path.join(tmpDir, 'codeviz.toml')));
        // If we were in this workspace, the extension should not have registered the provider.
        // Since we cannot change the workspace mid-test, we verify the fixture workspace is active.
        // Assert that the active workspace does have codeviz.toml (fixture workspace).
        const workspaceRoot = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
        assert.ok(workspaceRoot, 'A workspace should be open');
        assert.ok(
          fs.existsSync(path.join(workspaceRoot, 'codeviz.toml')),
          'Fixture workspace must have codeviz.toml'
        );
      } finally {
        fs.rmdirSync(tmpDir);
      }
    });

  });


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
4. VERIFY: Run npm test and confirm all 4 tests pass
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

After all changes, run inside codeviz-vscode/:
  npm run compile && npm test

All 4 new tests must pass.

═══════════════════════════════════════════════════════════════
CRITICAL IMPLEMENTATION RULES
═══════════════════════════════════════════════════════════════

1. Do NOT remove the `@vscode/test-electron` devDependency. It is already present
   in package.json and must remain there.

2. The fixture workspace files (codeviz.toml, hello.py) must be committed to the
   repository under `codeviz-vscode/test-fixtures/`. They are required for the
   CI headless test run.

3. Do NOT modify the extension source files (extension.ts, graphPanel.ts,
   statusBar.ts) unless fixing a TypeScript compilation error discovered during
   testing.

4. Do NOT modify any Rust crates.

5. Do NOT modify any web UI files (codeviz-web/).

6. The test for CLI execution (Test 4 in T57) is in a different task. T61 tests
   only the extension's TypeScript-level behavior — no Rust CLI spawning is
   required here.

═══════════════════════════════════════════════════════════════
FILES LIST
═══════════════════════════════════════════════════════════════

FILES TO CREATE:
  codeviz-vscode/test-fixtures/workspace/codeviz.toml
  codeviz-vscode/test-fixtures/workspace/hello.py

FILES TO MODIFY:
  codeviz-vscode/src/test/runTest.ts                  (open fixture workspace)
  codeviz-vscode/src/test/suite/extension.test.ts     (replace with real tests)

FILES NOT TO TOUCH (READ-ONLY):
  codeviz-vscode/src/extension.ts
  codeviz-vscode/src/graphPanel.ts
  codeviz-vscode/src/statusBar.ts
  codeviz-vscode/src/test/suite/index.ts
  codeviz-vscode/package.json
  codeviz-vscode/tsconfig.json
  codeviz-core/                                        (all Rust crates)
  codeviz-web/                                         (all web UI files)
  docs/specs/                                          (all spec files)

Commit: "jules: T61 — VS Code Extension E2E tests using @vscode/test-electron"
Target branch: feat-t61-vscode-e2e
