import { defineConfig, devices } from '@playwright/test';

// P4 TR-4.1 (axe-core, web) + TR-4.3 (visual regression, 3 breakpoints).
// Run against a built/served Innoframework web fixture. Rust CI does not run
// this by default (Node toolchain is optional/pinned here).
const baseURL = process.env.INNO_UI_BASE_URL || 'http://127.0.0.1:4173';

export default defineConfig({
  testDir: './tests',
  fullyParallel: true,
  retries: process.env.CI ? 1 : 0,
  reporter: [['list'], ['html', { open: 'never' }]],
  use: { baseURL, trace: 'on-first-retry' },
  projects: [
    { name: 'mobile-360', use: { ...devices['Desktop Chrome'], viewport: { width: 360, height: 740 } } },
    { name: 'tablet-768', use: { ...devices['Desktop Chrome'], viewport: { width: 768, height: 1024 } } },
    { name: 'desktop-1280', use: { ...devices['Desktop Chrome'], viewport: { width: 1280, height: 800 } } },
  ],
});
