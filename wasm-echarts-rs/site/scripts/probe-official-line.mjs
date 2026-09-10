/**
 * 在已启动的 Vite 上逐个打开官网折线示例，采集 window.__OFFICIAL_EXAMPLE_RESULT__。
 *
 * 用法：node scripts/probe-official-line.mjs [baseUrl]
 */
import { officialLineExamples } from '../echarts/examples/official-line-catalog.js';

const baseUrl = process.argv[2] || 'http://localhost:5175';

async function loadPlaywright() {
  try {
    return await import('playwright-core');
  } catch {
    throw new Error('需要 playwright-core：在 site 目录执行 npm install --no-save playwright-core');
  }
}

function launchBrowser(chromium) {
  const attempts = [
    { channel: 'msedge' },
    { channel: 'chrome' },
    { executablePath: 'C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe' },
    { executablePath: 'C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe' },
  ];
  let lastErr;
  const tryNext = async (index) => {
    if (index >= attempts.length) {
      throw lastErr || new Error('无法启动 Chromium / Edge');
    }
    try {
      return await chromium.launch({
        headless: true,
        ...attempts[index],
      });
    } catch (err) {
      lastErr = err;
      return tryNext(index + 1);
    }
  };
  return tryNext(0);
}

async function probeOne(page, item) {
  const url = `${baseUrl}/echarts/examples/${item.id}.html`;
  const pageErrors = [];
  const onPageError = (err) => pageErrors.push(String(err?.message || err));
  page.on('pageerror', onPageError);
  try {
    await page.goto(url, { waitUntil: 'domcontentloaded', timeout: 30000 });
    await page.waitForFunction(
      () => {
        const result = window.__OFFICIAL_EXAMPLE_RESULT__;
        return result && result.status && result.status !== 'pending';
      },
      { timeout: 20000 },
    ).catch(() => {});
    await new Promise((resolve) => setTimeout(resolve, 3500));
    const result = await page.evaluate(() => window.__OFFICIAL_EXAMPLE_RESULT__ || null);
    const overlay = await page.evaluate(() => document.querySelector('.preview-error')?.textContent || '');
    return {
      id: item.id,
      title: item.title,
      status: result?.status || (overlay || pageErrors.length ? 'error' : 'timeout'),
      error: result?.error || overlay || pageErrors[0] || (result ? '' : '等待结果超时'),
    };
  } finally {
    page.off('pageerror', onPageError);
  }
}

async function main() {
  const { chromium } = await loadPlaywright();
  const browser = await launchBrowser(chromium);
  const page = await browser.newPage();
  const rows = [];
  try {
    for (const item of officialLineExamples) {
      process.stdout.write(`${item.id} … `);
      const row = await probeOne(page, item);
      rows.push(row);
      console.log(row.status, row.error ? `- ${row.error.split('\n')[0]}` : '');
    }
  } finally {
    await browser.close();
  }

  const ok = rows.filter((r) => r.status === 'ok');
  const bad = rows.filter((r) => r.status !== 'ok');
  console.log('\n--- JSON ---');
  console.log(JSON.stringify({ ok, bad, rows }, null, 2));
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
