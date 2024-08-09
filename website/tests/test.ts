import { expect, test } from '@playwright/test';

test('No SessionToken Expect Login Screen', async ({ page }) => {
	await page.goto('/');
	await expect(page.getByRole('heading', { name: 'Lindahl Cabin' })).toBeVisible();
});

test('homepage has title and links to intro page', async ({ page }) => {
  await page.goto('/');
  await expect(page).toHaveTitle(/Cabin Website/);

  const getStarted = page.locator('#buttonDiv');
  await getStarted.click();
  await expect(page).toHaveURL('/login');
});

test('screenshot', async ({ page }) => {
	await page.goto('/');
	await page.screenshot({ path: './test-results/screenshot.png' });
  });