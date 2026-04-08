import { chromium, FullConfig } from '@playwright/test'

async function globalSetup(config: FullConfig) {
  const browser = await chromium.launch()
  const page = await browser.newPage()
  await page.goto(config.webServer!.url || 'http://localhost:5173')
  await page.close()
  await browser.close()
}

export default globalSetup
