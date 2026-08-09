import { test, expect } from '@playwright/test';

test('login flow works using E2E credentials', async ({ page }) => {
  // Navigate to the sign-in page
  await page.goto('/api/auth/signin');

  // Verify the custom Credentials provider is shown
  await expect(page.locator('button', { hasText: 'Sign in with E2E Test Account' })).toBeVisible();

  // Fill in the credentials
  await page.fill('input[name="username"]', 'testuser');
  await page.fill('input[name="password"]', 'password');

  // Click sign in using the correct button
  await page.locator('form', { has: page.locator('button', { hasText: 'Sign in with E2E Test Account' }) }).locator('button').click();

  // Wait for the page to redirect
  await page.waitForURL('http://localhost:3001/');

  // Verify there's no sign in error in the URL
  expect(page.url()).not.toContain('error=');

  // Access the protected /app route and make sure we don't get redirected back to login
  await page.goto('/app');

  // Test will wait to make sure redirection doesn't happen
  await page.waitForTimeout(500);
  expect(page.url()).toContain('/app');
});

test('unauthenticated users are redirected from /app', async ({ page }) => {
  await page.goto('/app');
  await page.waitForURL('**/api/auth/signin');
  expect(page.url()).toContain('/api/auth/signin');
});
