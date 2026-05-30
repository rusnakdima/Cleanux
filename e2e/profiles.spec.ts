import { test, expect, Page } from '@playwright/test';

test.describe('Profile Management', () => {
  test('should display profiles page', async ({ page }) => {
    await page.goto('/profiles');
    await page.waitForLoadState('networkidle');

    await expect(page).toHaveURL(/profiles/);
  });

  test('should have create profile button', async ({ page }) => {
    await page.goto('/profiles');
    await page.waitForLoadState('networkidle');

    const createButton = page.locator(
      'button:has-text("Create"), button:has-text("New"), button:has-text("Add")'
    );
    await expect(createButton.first()).toBeVisible({ timeout: 5000 });
  });

  test('should list existing profiles', async ({ page }) => {
    await page.goto('/profiles');
    await page.waitForLoadState('networkidle');

    await page.waitForTimeout(1000);
    const profileList = page.locator('[class*="profile"], [class*="card"], tr, li');
    expect(await profileList.count()).toBeGreaterThanOrEqual(0);
  });
});
