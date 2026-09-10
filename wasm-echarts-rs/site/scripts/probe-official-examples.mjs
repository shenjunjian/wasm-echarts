/**
 * 在已启动的 Vite 上逐个打开官网同步示例，采集 window.__OFFICIAL_EXAMPLE_RESULT__。
 *
 * 用法：
 *   node scripts/probe-official-examples.mjs --category line
 *   node scripts/probe-official-examples.mjs --category bar --category pie
 *   node scripts/probe-official-examples.mjs --base-url http://localhost:5175
 * 省略 --category 时探测 examples 目录里已有的全部 official-*-catalog.js。
 */
import { readdir } from 'node:fs/promises';
import { dirname, join, normalize, resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const examplesDir = join(__dirname, '..', 'echarts', 'examples');

export function parseArgs(argv) {
  const categories = [];
  let baseUrl = 'http://localhost:5175';
  for (let i = 0; i < argv.length; i++) {
    const arg = argv[i];
    if (arg === '--category' || arg === '-c') {
      const value = argv[++i];
      if (!value) {
        throw new Error(`${arg} 需要类别名，例如 --category bar`);
      }
      categories.push(value);
      continue;
    }
    if (arg.startsWith('--category=')) {
      categories.push(arg.slice('--category='.length));
      continue;
    }
    if (arg === '--base-url') {
      const value = argv[++i];
      if (!value) {
        throw new Error('--base-url 需要 URL');
      }
      baseUrl = value.replace(/\/$/, '');
      continue;
    }
    if (arg.startsWith('--base-url=')) {
      baseUrl = arg.slice('--base-url='.length).replace(/\/$/, '');
      continue;
    }
    if (/^https?:\/\//.test(arg)) {
      baseUrl = arg.replace(/\/$/, '');
      continue;
    }
    throw new Error(`未知参数: ${arg}`);
  }
  return { categories, baseUrl };
}

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

async function listCatalogCategories() {
  const names = await readdir(examplesDir);
  return names
    .map((name) => {
      const match = /^official-(.+)-catalog\.js$/.exec(name);
      return match ? match[1] : null;
    })
    .filter(Boolean)
    .sort();
}

async function loadCatalog(category) {
  const fileUrl = pathToFileURL(join(examplesDir, `official-${category}-catalog.js`)).href;
  const mod = await import(fileUrl);
  const examples = mod.officialExamples;
  if (!Array.isArray(examples)) {
    throw new Error(`official-${category}-catalog.js 缺少 officialExamples`);
  }
  return examples;
}

async function probeOne(page, baseUrl, item) {
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

export async function main(argv = process.argv.slice(2)) {
  const { categories: requested, baseUrl } = parseArgs(argv);
  const categories = requested.length ? requested : await listCatalogCategories();
  if (!categories.length) {
    throw new Error('没有可探测的 official-*-catalog.js，先跑 sync-official-examples.mjs');
  }

  const items = [];
  const seen = new Set();
  for (const category of categories) {
    const catalog = await loadCatalog(category);
    for (const item of catalog) {
      if (seen.has(item.id)) continue;
      seen.add(item.id);
      items.push(item);
    }
  }

  const { chromium } = await loadPlaywright();
  const browser = await launchBrowser(chromium);
  const page = await browser.newPage();
  const rows = [];
  try {
    console.log(`探测 ${items.length} 条（${categories.join(', ')}） @ ${baseUrl}`);
    for (const item of items) {
      process.stdout.write(`${item.id} … `);
      const row = await probeOne(page, baseUrl, item);
      rows.push(row);
      console.log(row.status, row.error ? `- ${row.error.split('\n')[0]}` : '');
    }
  } finally {
    await browser.close();
  }

  const ok = rows.filter((r) => r.status === 'ok');
  const bad = rows.filter((r) => r.status !== 'ok');
  console.log('\n--- JSON ---');
  console.log(JSON.stringify({ categories, ok, bad, rows }, null, 2));
}

function isDirectRun() {
  if (!process.argv[1]) return false;
  return (
    normalize(resolve(process.argv[1])).toLowerCase() ===
    normalize(fileURLToPath(import.meta.url)).toLowerCase()
  );
}

if (isDirectRun()) {
  main().catch((err) => {
    console.error(err);
    process.exit(1);
  });
}
