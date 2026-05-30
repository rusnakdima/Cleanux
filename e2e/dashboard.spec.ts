import { test, expect, Page } from '@playwright/test';

test.describe('Dashboard', () => {
  test('should display dashboard', async ({ page }) => {
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    await expect(page).toHaveURL(/dashboard/);
  });

  test('should display health score', async ({ page }) => {
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    const healthScore = page.locator('text=/\\d+/').first();
    await expect(healthScore).toBeVisible({ timeout: 5000 });
  });

  test('should display system stats widgets', async ({ page }) => {
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    const widgets = page.locator('[class*="widget"], [class*="card"]');
    expect(await widgets.count()).toBeGreaterThan(0);
  });
});
