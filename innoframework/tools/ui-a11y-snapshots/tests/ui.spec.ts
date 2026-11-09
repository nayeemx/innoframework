import AxeBuilder from '@axe-core/playwright';
import { expect, test } from '@playwright/test';

test('TR-4.1 web a11y smoke (axe-core, critical/serious)', async ({ page }) => {
  await page.goto('/');
  const results = await new AxeBuilder({ page })
    .withTags(['wcag2a', 'wcag2aa', 'wcag21a', 'wcag21aa'])
    .analyze();
  const severe = results.violations.filter((v) => ['critical', 'serious'].includes(v.impact || ''));
  expect(severe, JSON.stringify(severe, null, 2)).toEqual([]);
});

test('TR-4.3 visual snapshot baseline (3 breakpoints via projects)', async ({ page }) => {
  await page.goto('/');
  await expect(page).toHaveScreenshot('home.png', { fullPage: true, maxDiffPixelRatio: 0.02 });
});
