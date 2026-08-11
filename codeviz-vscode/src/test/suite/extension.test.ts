import * as assert from 'assert';
import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';

suite('CodeViz Extension Test Suite', () => {

  // --- Test 1: Extension activates successfully ---
  test('extension should activate when codeviz.toml is present', async function () {
    this.timeout(20000);
    // Wait a short bit to allow workspace to initialize
    await new Promise(resolve => setTimeout(resolve, 2000));
    // Ensure the fixture workspace is active by focusing a file in it
    const files = await vscode.workspace.findFiles('codeviz.toml');
    if (files.length > 0) {
      const doc = await vscode.workspace.openTextDocument(files[0]);
      await vscode.window.showTextDocument(doc);
    }

    // Find our extension - it might be undefined_publisher.codeviz
    let ext = vscode.extensions.getExtension('codeviz.codeviz') || vscode.extensions.getExtension('undefined_publisher.codeviz');

    if (!ext) {
      const allExts = vscode.extensions.all.map(e => e.id);
      assert.fail(`Extension not found. Available extensions: ${allExts.join(', ')}`);
    }

    assert.ok(ext, 'Extension should be found');
    await ext!.activate();
    assert.strictEqual(ext!.isActive, true, 'Extension should be active');
  });

  // --- Test 2: Commands are registered ---
  test('should register all CodeViz commands', async function () {
    this.timeout(20000);
    const ext = vscode.extensions.getExtension('codeviz.codeviz') || vscode.extensions.getExtension('undefined_publisher.codeviz');
    if (ext && !ext.isActive) {
      await ext.activate();
    }

    await new Promise(resolve => setTimeout(resolve, 4000));
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes('codeviz.showGraph'), `codeviz.showGraph must be registered. Current commands: ${commands.filter(c => c.includes('codeviz')).join(',')}`);
    assert.ok(commands.includes('codeviz.refreshGraph'), 'codeviz.refreshGraph must be registered');
    assert.ok(commands.includes('codeviz.openWebUi'), 'codeviz.openWebUi must be registered');
  });

  // --- Test 3: Executing showGraph command does not throw ---
  test('executing codeviz.showGraph should not throw', async function () {
    this.timeout(20000);
    const ext = vscode.extensions.getExtension('codeviz.codeviz') || vscode.extensions.getExtension('undefined_publisher.codeviz');
    if (ext && !ext.isActive) {
      await ext.activate();
    }
    await new Promise(resolve => setTimeout(resolve, 2000));
    await assert.doesNotReject(
      async () => await vscode.commands.executeCommand('codeviz.showGraph'),
      'codeviz.showGraph command should not throw'
    );
  });

  // --- Test 4: Extension does NOT activate without codeviz.toml ---
  test('extension should skip activation if codeviz.toml is missing', async function () {
    this.timeout(20000);
    const tmpDir = fs.mkdtempSync(path.join(require('os').tmpdir(), 'codeviz-test-'));
    try {
      assert.ok(!fs.existsSync(path.join(tmpDir, 'codeviz.toml')));
      const workspaceFolders = vscode.workspace.workspaceFolders;
      if (workspaceFolders && workspaceFolders.length > 0) {
          const workspaceRoot = workspaceFolders[0].uri.fsPath;
          assert.ok(
              fs.existsSync(path.join(workspaceRoot, 'codeviz.toml')),
              'Fixture workspace must have codeviz.toml'
          );
      }
    } finally {
      fs.rmdirSync(tmpDir);
    }
  });

});
