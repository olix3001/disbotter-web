import { test, expect, type Page } from '@playwright/test';

async function addNode(page: Page, name: string, position = { x: 100, y: 100 }) {
	// Right click on editor
	await page.click('.editor', { button: 'right', position });

	// Expect context menu to be visible
	await expect(page.locator('.editor-context-search')).toBeVisible();

	// Search for name
	await page.locator('.editor-context-search input').fill(name);

	// Expect name to be visible
	await expect(page.locator('.ec-node', { hasText: name })).toBeVisible();

	// Click name
	await page.locator('.ec-node', { hasText: name }).click();
}

test('create and move nodes', async ({ page }) => {
	await page.goto('/editor');

	// Click button with text "New"
	await page.getByRole('button', { name: 'New' }).click();

	// Get editor
	const editor = await page.locator('.editor').first();

	await expect(editor).toBeVisible();

	// Add node
	await addNode(page, 'On Command');

	// Move nodes
	const node = await page.locator('.node-view').first();
	const nodeBB = node.boundingBox();

	// Drag node to new position
	await node.dragTo(editor, { targetPosition: { x: 200, y: 200 } });

	// Expect node to be moved
	await expect(nodeBB).not.toEqual(await node.boundingBox());

	// Expect node to be selected
	await expect(page.locator('.node-view')).toHaveClass(/nv-selected/);
});

test('connect and disconnect nodes', async ({ page }) => {
	await page.goto('/editor');

	// Click button with text "New"
	await page.getByRole('button', { name: 'New' }).click();

	// Get editor
	const editor = await page.locator('.editor').first();
	await expect(editor).toBeVisible();

	// Add a few nodes
	await addNode(page, 'On Command');
	await addNode(page, 'Reply', { x: 300, y: 200 });

	// Get first node
	const nodeA = await page.locator('.node-view', { hasText: 'On Command' }).first();
	const nodeB = await page.locator('.node-view', { hasText: 'Reply' }).first();

	const nodeABB = await nodeA.boundingBox();
	const nodeBBB = await nodeB.boundingBox();

	// Get node connectors
	const nodeAConnector = await nodeA.locator('.nc').first();
	const nodeBConnector = await nodeB.locator('.nc').first();

	// Drag nodeA connector to nodeB connector
	await nodeAConnector.dragTo(nodeBConnector);

	// Expect connection to be created
	await expect(page.locator('.editor-connections').locator('path')).toBeVisible();

	// Expect nodes not to move
	await expect(nodeABB).toEqual(await nodeA.boundingBox());
	await expect(nodeBBB).toEqual(await nodeB.boundingBox());

	await nodeBConnector.dragTo(nodeAConnector);

	// Expect connection to be removed
	await expect(page.locator('.editor-connections').locator('path')).not.toBeVisible();
});

test('delete nodes', async ({ page }) => {
	await page.goto('/editor');

	// Click button with text "New"
	await page.getByRole('button', { name: 'New' }).click();

	// Get editor
	const editor = await page.locator('.editor').first();
	await expect(editor).toBeVisible();

	// Add a few nodes
	await addNode(page, 'On Command');
	await addNode(page, 'On Command', { x: 200, y: 200 });
	await addNode(page, 'On Command', { x: 300, y: 300 });

	// All nodes should be visible
	await expect(page.locator('.node-view')).toHaveCount(3);

	// Get first node
	const node = await page.locator('.node-view').first();

	// Click node
	await node.click();

	// Expect node to be selected
	await expect(node).toHaveClass(/nv-selected/);

	// Press delete key
	await page.keyboard.press('Delete');

	// Expect node to be removed
	await expect(page.locator('.node-view')).toHaveCount(2);
});

test('dont allow flow out to flow out connections', async ({ page }) => {
	await page.goto('/editor');

	// Click button with text "New"
	await page.getByRole('button', { name: 'New' }).click();

	// Get editor
	const editor = await page.locator('.editor').first();
	await expect(editor).toBeVisible();

	// Add a few nodes
	await addNode(page, 'On Command');
	await addNode(page, 'On Command', { x: 200, y: 200 });

	// Get first and second node's connector
	const nodeAConnector = await page
		.locator('.node-view', { hasText: 'On Command' })
		.first()
		.locator('.nc')
		.first();
	const nodeBConnector = await page
		.locator('.node-view', { hasText: 'On Command' })
		.first()
		.locator('.nc')
		.first();

	// Drag nodeA connector to nodeB connector
	await nodeAConnector.dragTo(nodeBConnector);

	// Expect connection to not be created
	await expect(page.locator('.editor-connections').locator('path')).not.toBeVisible();
});

test('allow only same types to connect', async ({ page }) => {
	await page.goto('/editor');

	// Click button with text "New"
	await page.getByRole('button', { name: 'New' }).click();

	// Get editor
	const editor = await page.locator('.editor').first();
	await expect(editor).toBeVisible();

	// Add a few nodes
	await addNode(page, 'Create Text Message');
	await addNode(page, 'Reply', { x: 400, y: 200 });

	const nodeAOut = page.locator('.nvb-outputs > .nf-block > .nc').first();
	const nodeBIn = page
		.locator('div:nth-child(3) > .node-view-body > .nvb-inputs > div > .nc')
		.first();

	// Drag nodeA connector to nodeB connector
	await nodeAOut.dragTo(nodeBIn);

	// Expect connection to not be created
	await expect(page.locator('.editor-connections').locator('path')).not.toBeVisible();

	// Now try the other way around
	await nodeBIn.dragTo(nodeAOut);

	// Expect connection to not be created
	await expect(page.locator('.editor-connections').locator('path')).not.toBeVisible();

	// And correct way
	const nodeBIn2 = page.locator('div:nth-child(2) > .nc');

	// Drag nodeA connector to nodeB connector
	await nodeAOut.dragTo(nodeBIn2);

	// Expect connection to be created
	await expect(page.locator('.editor-connections').locator('path')).toBeVisible();
});
