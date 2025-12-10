import { test, expect } from '@playwright/test';

test.describe('Authentication Flow', () => {
    const timestamp = Date.now();
    const testEmail = `test${timestamp}@example.com`;
    const testPassword = 'Password123!';

    test('should register a new user', async ({ page }) => {
        await page.goto('/auth/register');

        // Check if page loaded
        await expect(page).toHaveTitle(/BeppoFit/);

        await page.locator('ion-input[formControlName="email"]').waitFor();
        // Use fill on the ion-input itself might fail.
        // Let's try the recommended way for Ionic:
        await page.locator('ion-input[formControlName="email"] input').fill(testEmail);
        await page.locator('ion-input[formControlName="password"] input').fill(testPassword);

        // Intercept register request to verify it happens
        const registerPromise = page.waitForResponse(resp =>
            resp.url().includes('/auth/register') && resp.status() === 200
        );

        await page.click('ion-button[type="submit"]');

        await registerPromise;

        // Should navigate to home
        await expect(page).toHaveURL(/.*\/home/);
    });

    test('should login with the registered user', async ({ page }) => {
        // Note: Since we are running against a real backend, we rely on the user existing.
        // If tests run in parallel or order is not guaranteed, we might need to seed data or mock.
        // Ideally we register a fresh user per test or use a global setup. 
        // For this simple verify, we'll try to login with the same user if persistence allows, 
        // OR just re-register and ignore 409, OR just register a new one for login test too.
        // Let's register a new one for this specific test to be safe/independent.
        const loginTestEmail = `login${Date.now()}@example.com`;

        // Register first
        await page.goto('/auth/register');
        await page.locator('ion-input[formControlName="email"] input').fill(loginTestEmail);
        await page.locator('ion-input[formControlName="password"] input').fill(testPassword);

        const registerPromise = page.waitForResponse(resp =>
            resp.url().includes('/auth/register') && resp.status() === 200
        );
        await page.click('ion-button[type="submit"]');
        await registerPromise;

        await expect(page).toHaveURL(/.*\/home/);

        // Logout
        // Assuming there is a logout button in home or we can just go to login
        await page.goto('/auth/login');

        // Login
        await page.locator('ion-input[formControlName="email"] input').fill(loginTestEmail);
        await page.locator('ion-input[formControlName="password"] input').fill(testPassword);

        const loginPromise = page.waitForResponse(resp =>
            resp.url().includes('/auth/login') && resp.status() === 200
        );

        await page.click('ion-button[type="submit"]');
        await loginPromise;

        await expect(page).toHaveURL(/.*\/home/);
    });

    test('should show error for invalid login', async ({ page }) => {
        await page.goto('/auth/login');
        await page.locator('ion-input[formControlName="email"] input').fill('invalid@example.com');
        await page.locator('ion-input[formControlName="password"] input').fill('WrongPass!');

        await page.click('ion-button[type="submit"]');

        // Check for error message
        await expect(page.locator('.error-message')).toBeVisible({ timeout: 5000 });
    });

    test('should show correct error for non-existing user login', async ({ page }) => {
        await page.goto('/auth/login');
        await page.locator('ion-input[formControlName="email"] input').fill('nonexisting@example.com');
        await page.locator('ion-input[formControlName="password"] input').fill('Password123!');

        await page.click('ion-button[type="submit"]');

        // Check for specific error message
        const errorMessage = page.locator('.error-message');
        await expect(errorMessage).toBeVisible();
        await expect(errorMessage).toContainText('Unknown e-mail');
    });

    test('should handle existing unverified user registration (resend token)', async ({ page }) => {
        // Register a user first (or use the one from previous test if order guaranteed, but safest to make a new one)
        const dupEmail = `dup${Date.now()}@example.com`;

        // First registration
        await page.goto('/auth/register');
        await page.locator('ion-input[formControlName="email"] input').fill(dupEmail);
        await page.locator('ion-input[formControlName="password"] input').fill(testPassword);
        let responsePromise = page.waitForResponse(resp => resp.url().includes('/register') && resp.status() === 200);
        await page.click('ion-button[type="submit"]');
        await responsePromise;

        // Second registration with same email
        await page.goto('/auth/register');
        await page.locator('ion-input[formControlName="email"] input').fill(dupEmail);
        await page.locator('ion-input[formControlName="password"] input').fill(testPassword);

        // Expect 200 OK (Resend Token) instead of 409 Conflict
        responsePromise = page.waitForResponse(resp => resp.url().includes('/register') && resp.status() === 200);
        await page.click('ion-button[type="submit"]');
        await responsePromise;

        // Should redirect to home (auto-login allowed for unverified users now)
        await expect(page).toHaveURL(/.*\/home/);
    });

    test('should allow account deletion', async ({ page }) => {
        const deleteEmail = `delete${Date.now()}@example.com`;

        // Register & Login
        await page.goto('/auth/register');
        await page.locator('ion-input[formControlName="email"] input').fill(deleteEmail);
        await page.locator('ion-input[formControlName="password"] input').fill(testPassword);
        const registerPromise = page.waitForResponse(resp => resp.url().includes('/register') && resp.status() === 200);
        await page.click('ion-button[type="submit"]');
        await registerPromise;

        // Login (required for delete)
        await page.goto('/auth/login');
        await page.locator('ion-input[formControlName="email"] input').fill(deleteEmail);
        await page.locator('ion-input[formControlName="password"] input').fill(testPassword);
        const loginPromise = page.waitForResponse(resp => resp.url().includes('/login') && resp.status() === 200);
        await page.click('ion-button[type="submit"]');
        await loginPromise;

        // Navigate to account
        await page.goto('/account');

        // Handle confirm dialog
        page.on('dialog', dialog => dialog.accept());

        // Click delete
        // Assuming there is a button with text "Delete Account"
        // Wait for button
        await page.getByText('Delete Account').click();

        // Expect logout/redirect to login
        await expect(page).toHaveURL(/.*\/auth\/login/);

        // Try Login Again -> Should fail
        await page.locator('ion-input[formControlName="email"] input').fill(deleteEmail);
        await page.locator('ion-input[formControlName="password"] input').fill(testPassword);
        const failedLoginPromise = page.waitForResponse(resp => resp.url().includes('/login') && resp.status() === 401);
        await page.click('ion-button[type="submit"]');
        await failedLoginPromise;
    });
});
