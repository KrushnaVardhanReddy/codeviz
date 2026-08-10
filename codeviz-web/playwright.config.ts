import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './e2e',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: 'html',
  use: {
    baseURL: 'http://localhost:3001',
    trace: 'on-first-retry',
  },
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
  ],
  webServer: [
    {
      command: 'surreal start --username root --password root --default-namespace codeviz --default-database main --import-file ./seed.surql memory',
      url: 'http://127.0.0.1:8000/health',
      reuseExistingServer: false,
      timeout: 10_000,
    },
    {
      command: 'npm run dev -- -p 3001',
      url: 'http://localhost:3001',
      reuseExistingServer: !process.env.CI,
      env: {
        SURREALDB_URL: 'http://127.0.0.1:8000/rpc',
        SURREALDB_USER: 'root',
        SURREALDB_PASS: 'root',
        SURREALDB_NS: 'codeviz',
        SURREALDB_DB: 'main',
        AUTH_SECRET: 'e2e_fallback_secret',
      },
    },
  ],
});
