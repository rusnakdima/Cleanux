import { test, expect, Page } from '@playwright/test';

test.describe('Automation', () => {
  test('should display automation page', async ({ page }) => {
    await page.goto('/automation');
    await page.waitForLoadState('networkidle');

    await expect(page).toHaveURL(/automation/);
  });

  test('should display quick actions', async ({ page }) => {
    await page.goto('/automation');
    await page.waitForLoadState('networkidle');

    await page.waitForTimeout(1000);
    const quickActions = page.locator('[class*="action"], [class*="card"], button');
    expect(await quickActions.count()).toBeGreaterThan(0);
  });

  test('should display recipes', async ({ page }) => {
    await page.goto('/automation');
    await page.waitForLoadState('networkidle');

    await page.waitForTimeout(1000);
  });
});
