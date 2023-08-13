import { test, expect, type Page } from '@playwright/test';

test('new button creates a command', async ({ page }) => {
	await page.goto('/editor');

	// Click button with text "New"
	await page.getByRole('button', { name: 'New' }).click();

	// Check that the command was created
	await expect(page.getByText('new-command')).toBeVisible();

	// Check that the command is selected
	await expect(page.locator('.selected')).toHaveText('new-command');
});

async function clickMenu(page: Page, name: string, optionName: string) {
	// Click on the file menu
	const fileMenu = await page.getByRole('button', { name });

	// Click on the file menu
	await fileMenu.hover();

	// Expect button to be visible
	await expect(page.getByRole('button', { name: optionName }).first()).toBeVisible();

	// Click on the button
	await page.getByRole('button', { name: optionName }).first().click();
}

test('saving project opens file dialog', async ({ page }) => {
	await page.goto('/editor');

	const download = page.waitForEvent('download');

	// Click on the save button
	await clickMenu(page, 'File', 'Save');

	// Expect file dialog to open
	await expect(await download).toBeTruthy();
});

test('opening project opens file dialog', async ({ page }) => {
	await page.goto('/editor');

	const fileDialog = page.waitForEvent('filechooser');

	// Click on the save button
	await clickMenu(page, 'File', 'Open');

	// Expect file dialog to open
	await expect(await fileDialog).toBeTruthy();
});
