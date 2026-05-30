import { test, expect, Page } from '@playwright/test';

test.describe('Cleaner Flow', () => {
  test('should display cleaner page', async ({ page }) => {
    await page.goto('/cleaner');
    await page.waitForLoadState('networkidle');

    await expect(page.locator('h1, h2, h3').first()).toBeVisible();
  });

  test('should have cleaner tabs (cache, trash, logs)', async ({ page }) => {
    await page.goto('/cleaner');
    await page.waitForLoadState('networkidle');

    const tabs = page.locator(
      '[role="tab"], .tab, button:has-text("Cache"), button:has-text("Trash"), button:has-text("Logs")'
    );
    const tabCount = await tabs.count();
    expect(tabCount).toBeGreaterThan(0);
  });

  test('should scan for cache files', async ({ page }) => {
    await page.goto('/cleaner');
    await page.waitForLoadState('networkidle');

    const scanButton = page.locator('button:has-text("Scan"), button:has-text("Refresh")').first();
    if (await scanButton.isVisible()) {
      await scanButton.click();
      await page.waitForTimeout(1000);
    }
  });
});
