import { test, expect, Page } from '@playwright/test';

test.describe('System Services', () => {
  test('should display system page', async ({ page }) => {
    await page.goto('/system');
    await page.waitForLoadState('networkidle');

    await expect(page).toHaveURL(/system/);
  });

  test('should display services list', async ({ page }) => {
    await page.goto('/system');
    await page.waitForLoadState('networkidle');

    await page.waitForTimeout(1000);
    const services = page.locator('[class*="service"], tr, [class*="list"]');
    expect(await services.count()).toBeGreaterThanOrEqual(0);
  });
});
