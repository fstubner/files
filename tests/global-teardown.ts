import { chromium } from '@playwright/test'

async function globalTeardown() {
  const browser = await chromium.launch()
  await browser.close()
}

export default globalTeardown
