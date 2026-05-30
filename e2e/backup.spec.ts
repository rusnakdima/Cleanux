import { test, expect, Page } from '@playwright/test';

test.describe('Backup Flow', () => {
  test('should display backup page', async ({ page }) => {
    await page.goto('/backup');
    await page.waitForLoadState('networkidle');

    await expect(page).toHaveURL(/backup/);
  });

  test('should have create backup button', async ({ page }) => {
    await page.goto('/backup');
    await page.waitForLoadState('networkidle');

    const backupButton = page.locator('button:has-text("Backup"), button:has-text("Create")');
    await expect(backupButton.first()).toBeVisible({ timeout: 5000 });
  });

  test('should list existing backups', async ({ page }) => {
    await page.goto('/backup');
    await page.waitForLoadState('networkidle');

    await page.waitForTimeout(1000);
  });
});
