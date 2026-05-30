import { test, expect, Page } from '@playwright/test';

test.describe('Settings', () => {
  test('should display settings page', async ({ page }) => {
    await page.goto('/settings');
    await page.waitForLoadState('networkidle');

    await expect(page).toHaveURL(/settings/);
  });

  test('should have theme toggle', async ({ page }) => {
    await page.goto('/settings');
    await page.waitForLoadState('networkidle');

    const themeToggle = page.locator(
      'button:has-text("Dark"), button:has-text("Light"), [class*="toggle"], input[type="checkbox"]'
    );
    const count = await themeToggle.count();
    expect(count).toBeGreaterThan(0);
  });

  test('should display language selector', async ({ page }) => {
    await page.goto('/settings');
    await page.waitForLoadState('networkidle');

    const languageSelect = page.locator('select, [role="combobox"], [class*="language"]');
    await expect(languageSelect.first()).toBeVisible({ timeout: 5000 });
  });
});
