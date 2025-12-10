import { test, expect } from '@playwright/test';

test('has title', async ({ page }) => {
    await page.goto('/');

    // Expect a title "to contain" a substring.
    // Note: Adjust the expected title based on your actual app title
    await expect(page).toHaveTitle(/BeppoFit/);
});
