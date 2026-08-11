import { test, expect } from '@playwright/test';
import { spawnSync } from 'child_process';
import * as os from 'os';
import * as fs from 'fs';
import * as path from 'path';

test.describe('MVP v1 E2E Test Suite', () => {
  // Test 1: Rust CLI spawns and produces Mermaid output
  test('should spawn the Rust CLI and produce a valid Mermaid diagram', async () => {
    // 1. Create a temporary directory.
    const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), 'codeviz-e2e-'));
    
    try {
      // 2. Write a minimal codeviz.toml to it
      fs.writeFileSync(path.join(tmpDir, 'codeviz.toml'), '[project]\n');

      // 3. Write a simple Python file
      fs.writeFileSync(path.join(tmpDir, 'main.py'), 'def hello():\n    pass\n');

      // Check if CLI binary exists
      const cliPath = path.resolve('../target/release/codeviz');
      if (!fs.existsSync(cliPath)) {
        test.skip(true, 'Rust CLI binary not found at target/release/codeviz, skipping test');
        return;
      }

      // 4. Spawn the Rust CLI
      const { stdout, stderr, status } = spawnSync(
        cliPath,
        ['run', '--path', tmpDir, '--diagram', 'module'],
        { cwd: path.resolve('..') }
      );

      // 5. Assert: exitCode === 0
      expect(status).toBe(0);

      // 6. Assert: stdout contains "graph TD" or "graph LR" (Mermaid module graph header)
      const output = stdout.toString();
      expect(output).toContain('graph');
    } finally {
      // 7. Clean up the temporary directory.
      fs.rmSync(tmpDir, { recursive: true, force: true });
    }
  });

  // Test 2: Call Path Explorer opens in the Web UI
  test('should open the Call Path Explorer when Trace button is clicked', async ({ page }) => {
    await page.goto('/');

    await page.waitForSelector('[data-testid="graph-canvas"]', { state: 'visible' });

    // The data-type is not guaranteed to be "Function", it depends on how the graph converts it.
    // Let's wait for the first node, and try to click it.
    const backupNode = page.locator('.react-flow__node').first();
    await backupNode.waitFor({ state: 'visible', timeout: 15000 });
    await backupNode.click();
    
    const detailPanel = page.locator('[data-testid="detail-panel"]');
    await detailPanel.waitFor({ state: 'visible' });

    const traceBtn = page.locator('[data-testid="trace-paths-btn"]');
    
    // The instructions state: "Assert that a button containing text "Trace" or data-testid="trace-btn" is visible in the panel."
    await traceBtn.waitFor({ state: 'visible', timeout: 5000 }).catch(() => {
        // If it doesn't appear, the first node wasn't a function, or the logic is broken.
    });

    if (await traceBtn.isVisible()) {
        await traceBtn.click();
        const callPathExplorer = page.locator('[data-testid="call-path-explorer"]');
        await callPathExplorer.waitFor({ state: 'visible' });
        await expect(callPathExplorer).toBeVisible();
    } else {
        await expect(traceBtn).toBeVisible();
    }
  });

  // Test 3: Code Playground parses source and renders a graph
  test('should render a graph from source code typed into the playground', async ({ page }) => {
    await page.goto('/playground');

    // Wait for playground editor
    await page.waitForSelector('[data-testid="playground-editor"]', { state: 'visible' });

    // Click inside the editor to focus it
    const editor = page.locator('.monaco-editor').first();
    await editor.waitFor({ state: 'visible' });
    await editor.click();
    
    // Clear and type using Playwright's keyboard
    const isMac = os.platform() === 'darwin';
    await page.keyboard.press(isMac ? 'Meta+A' : 'Control+A');
    await page.keyboard.press('Backspace');
    await page.keyboard.type('def hello():\n    pass\ndef world():\n    hello()');

    // Wait for at least one react-flow__node to be rendered.
    const firstNode = page.locator('.react-flow__node').first();
    await firstNode.waitFor({ state: 'visible', timeout: 15000 });

    // Assert that at least one .react-flow__node is rendered
    const nodeCount = await page.locator('.react-flow__node').count();
    expect(nodeCount).toBeGreaterThan(0);
  });

  // Test 4: CFG side panel appears for a function node
  test('should show the CFG panel when a function node is clicked', async ({ page }) => {
    await page.goto('/');

    await page.waitForSelector('[data-testid="graph-canvas"]', { state: 'visible' });

    // The instructions state: "click the first available node and check if cfg-panel appears."
    const firstNode = page.locator('.react-flow__node').first();
    await firstNode.waitFor({ state: 'visible', timeout: 15000 });
    await firstNode.click();

    const detailPanel = page.locator('[data-testid="detail-panel"]');
    await detailPanel.waitFor({ state: 'visible' });

    const cfgPanel = page.locator('[data-testid="cfg-panel"]');
    await cfgPanel.waitFor({ state: 'visible', timeout: 5000 }).catch(() => {});
    
    if (await cfgPanel.isVisible()) {
        await expect(cfgPanel).toBeVisible();
    } else {
        await expect(cfgPanel).toBeVisible();
    }
  });
});
