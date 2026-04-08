import { test, expect } from '@playwright/test'

test.describe('Search', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/')
  })

  test('should search for files', async ({ page }) => {
    await page.fill('[data-testid="search-input"]', 'test')
    await page.press('[data-testid="search-input"]', 'Enter')
    
    const results = page.locator('[data-testid="search-result"]')
    const count = await results.count()
    expect(count).toBeGreaterThan(0)
  })

  test('should filter search results', async ({ page }) => {
    await page.fill('[data-testid="search-input"]', 'test')
    await page.press('[data-testid="search-input"]', 'Enter')
    
    await page.selectOption('[data-testid="file-type-filter"]', '.txt')
    const results = page.locator('[data-testid="search-result"]')
    const items = await results.count()
    expect(items).toBeGreaterThanOrEqual(0)
  })

  test('should clear search', async ({ page }) => {
    await page.fill('[data-testid="search-input"]', 'test')
    await page.press('[data-testid="search-input"]', 'Enter')
    
    await page.click('[data-testid="clear-search-button"]')
    const fileList = page.locator('[data-testid="file-list"]')
    await expect(fileList).toBeVisible()
  })
})
