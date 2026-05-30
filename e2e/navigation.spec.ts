import { test, expect, Page } from '@playwright/test';

test.describe('Navigation', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
  });

  test('should navigate to dashboard', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page).toHaveURL(/dashboard/);
  });

  test('should navigate to cleaner', async ({ page }) => {
    await page.click('a[href="/cleaner"]');
    await expect(page).toHaveURL(/cleaner/);
  });

  test('should navigate to settings', async ({ page }) => {
    await page.click('a[href="/settings"]');
    await expect(page).toHaveURL(/settings/);
  });

  test('should navigate to backup', async ({ page }) => {
    await page.click('a[href="/backup"]');
    await expect(page).toHaveURL(/backup/);
  });

  test('should navigate to profiles', async ({ page }) => {
    await page.click('a[href="/profiles"]');
    await expect(page).toHaveURL(/profiles/);
  });

  test('should navigate to automation', async ({ page }) => {
    await page.click('a[href="/automation"]');
    await expect(page).toHaveURL(/automation/);
  });

  test('should navigate to memory optimizer', async ({ page }) => {
    await page.click('a[href="/memory-optimizer"]');
    await expect(page).toHaveURL(/memory-optimizer/);
  });

  test('should navigate to system', async ({ page }) => {
    await page.click('a[href="/system"]');
    await expect(page).toHaveURL(/system/);
  });
});
