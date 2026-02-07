import { chromium } from 'playwright';

const url = process.argv[2] || 'http://127.0.0.1:8080';
const out = process.argv[3] || 'screenshot.png';

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
await page.goto(url, { waitUntil: 'networkidle' });
// Wait extra for WASM to mount
await page.waitForTimeout(3000);
await page.screenshot({ path: out, fullPage: true });
console.log(`Screenshot saved to ${out}`);
await browser.close();
