import { test, expect } from '@playwright/test';

test('playground loads and displays layout', async ({ page }) => {
  await page.goto('/playground');

  // Wait for loading to finish and split panes to be visible
  await expect(page.locator('text=Language:')).toBeVisible();
  await expect(page.locator('text=Graph Preview')).toBeVisible();

  // Try to toggle language
  await page.selectOption('select', 'typescript');

  // Note: we can't fully test Monaco or Canvas content easily in this simple test,
  // but we can ensure the layout renders without crashing.
});
