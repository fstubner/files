import { test, expect } from '@playwright/test'
import path from 'path'

test.describe('File Operations', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/')
  })

  test('should list files in directory', async ({ page }) => {
    await page.click('[data-testid="folder-input"]')
    await page.fill('[data-testid="folder-input"]', path.join(__dirname, '../fixtures'))
    await page.press('[data-testid="folder-input"]', 'Enter')
    
    const fileList = await page.locator('[data-testid="file-list"]')
    await expect(fileList).toBeVisible()
    
    const items = await page.locator('[data-testid="file-item"]').count()
    expect(items).toBeGreaterThan(0)
  })

  test('should display file details', async ({ page }) => {
    const fileItem = page.locator('[data-testid="file-item"]').first()
    await fileItem.click()
    
    const details = page.locator('[data-testid="file-details"]')
    await expect(details).toBeVisible()
  })

  test('should copy file', async ({ page }) => {
    const fileItem = page.locator('[data-testid="file-item"]').first()
    await fileItem.click()
    await page.click('[data-testid="copy-button"]')
    
    const notification = page.locator('[data-testid="notification"]')
    await expect(notification).toContainText('copied')
  })
})
