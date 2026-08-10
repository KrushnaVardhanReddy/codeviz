import { test, expect } from '@playwright/test';

test('unauthenticated users are redirected from /app', async ({ page }) => {
  await page.goto('/app');
  await page.waitForURL('**/api/auth/signin*');
  expect(page.url()).toContain('/api/auth/signin');
});
