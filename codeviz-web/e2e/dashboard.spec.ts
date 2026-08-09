import { test, expect } from '@playwright/test';

test.describe('Dashboard E2E', () => {
  test('should render graph canvas and open detail panel on node click', async ({ page }) => {
    await page.goto('/');

    // Wait for the graph canvas to be visible
    const canvas = page.getByTestId('graph-canvas');
    await expect(canvas).toBeVisible();

    // Check that nodes are rendered (React Flow renders `.react-flow__node`)
    await expect(page.locator('.react-flow__node').first()).toBeVisible();

    // Click on a specific node that was rendered
    const node = page.getByTestId('node-App.tsx');
    await expect(node).toBeVisible();
    await node.click({ force: true }); // Use force true if node is covered by something else

    // Assert that the detail panel opens
    const detailPanel = page.getByTestId('detail-panel');
    await expect(detailPanel).toBeVisible();

    // Assert that the title in the detail panel is correct
    const panelTitle = page.getByTestId('detail-panel-title');
    await expect(panelTitle).toHaveText('App.tsx');

    // Close the panel
    const closeBtn = page.getByTestId('close-panel-btn');
    await closeBtn.click();

    // After closing, detail panel should either not be visible or have translate-x-full
    await expect(detailPanel).toHaveClass(/translate-x-full/);
  });
});
