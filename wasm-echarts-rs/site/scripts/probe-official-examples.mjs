/**
 * 在已启动的 Vite 上逐个打开官网同步示例，看 canvas 是否画出像素与 `.preview-error`。
 *
 * 用法：
 *   node scripts/probe-official-examples.mjs --category line
 *   node scripts/probe-official-examples.mjs --category bar --category pie
 *   node scripts/probe-official-examples.mjs --base-url http://localhost:5175
 *   node scripts/probe-official-examples.mjs --out probe-full.json
 * 省略 --category 时探测 examples 目录里已有的全部 official-*-catalog.js。
 */
import { readdir, writeFile } from 'node:fs/promises';
import { dirname, join, normalize, resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const examplesDir = join(__dirname, '..', 'echarts', 'examples');

export function parseArgs(argv) {
  const categories = [];
  let baseUrl = 'http://localhost:5175';
  let outPath = null;
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
    if (arg === '--out') {
      const value = argv[++i];
      if (!value) {
        throw new Error('--out 需要文件路径');
      }
      outPath = value;
      continue;
    }
    if (arg.startsWith('--out=')) {
      outPath = arg.slice('--out='.length);
      continue;
    }
    if (/^https?:\/\//.test(arg)) {
      baseUrl = arg.replace(/\/$/, '');
      continue;
    }
    throw new Error(`未知参数: ${arg}`);
  }
  return { categories, baseUrl, outPath };
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

const EXAMPLE_TIMEOUT_MS = 40000;

function withTimeout(promise, ms, timeoutValue) {
  let timer;
  const timeout = new Promise((resolve) => {
    timer = setTimeout(() => resolve({ timedOut: true, value: timeoutValue }), ms);
  });
  return Promise.race([
    promise.then((value) => ({ timedOut: false, value })),
    timeout,
  ]).finally(() => clearTimeout(timer));
}

async function recycleBrowser(browser, chromium) {
  try {
    const proc = browser.process?.();
    await Promise.race([
      browser.close(),
      new Promise((resolve) => setTimeout(resolve, 4000)),
    ]);
    proc?.kill?.();
  } catch {
    try {
      browser.process?.()?.kill?.();
    } catch {
      // ignore
    }
  }
  const next = await launchBrowser(chromium);
  return { browser: next, page: await next.newPage() };
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
        if (document.querySelector('.preview-error')?.textContent) return true;
        const canvas = document.getElementById('canvas');
        if (!canvas?.width || !canvas?.height) return false;
        const ctx = canvas.getContext('2d');
        if (!ctx) return false;
        const { width, height } = canvas;
        const data = ctx.getImageData(0, 0, width, height).data;
        const step = Math.max(1, Math.floor(Math.min(width, height) / 64));
        for (let y = 0; y < height; y += step) {
          for (let x = 0; x < width; x += step) {
            if (data[(y * width + x) * 4 + 3] > 8) return true;
          }
        }
        return false;
      },
      { timeout: 20000 },
    ).catch(() => {});
    await new Promise((resolve) => setTimeout(resolve, 3500));
    const snapshot = await page.evaluate(() => {
      const overlay = document.querySelector('.preview-error')?.textContent || '';
      const canvas = document.getElementById('canvas');
      if (!canvas?.width || !canvas?.height) {
        return { overlay, painted: false };
      }
      const ctx = canvas.getContext('2d');
      if (!ctx) {
        return { overlay, painted: false };
      }
      const { width, height } = canvas;
      const data = ctx.getImageData(0, 0, width, height).data;
      const step = Math.max(1, Math.floor(Math.min(width, height) / 64));
      for (let y = 0; y < height; y += step) {
        for (let x = 0; x < width; x += step) {
          if (data[(y * width + x) * 4 + 3] > 8) {
            return { overlay, painted: true };
          }
        }
      }
      return { overlay, painted: false };
    });
    const overlay = snapshot.overlay || '';
    if (overlay || pageErrors.length) {
      return {
        id: item.id,
        title: item.title,
        status: 'error',
        error: overlay || pageErrors[0],
      };
    }
    if (snapshot.painted) {
      return {
        id: item.id,
        title: item.title,
        status: 'ok',
        error: '',
      };
    }
    return {
      id: item.id,
      title: item.title,
      status: 'timeout',
      error: '等待结果超时',
    };
  } finally {
    page.off('pageerror', onPageError);
  }
}

export async function main(argv = process.argv.slice(2)) {
  const { categories: requested, baseUrl, outPath } = parseArgs(argv);
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
  let browser = await launchBrowser(chromium);
  let page = await browser.newPage();
  const rows = [];
  try {
    console.log(`探测 ${items.length} 条（${categories.join(', ')}） @ ${baseUrl}`);
    for (const item of items) {
      process.stdout.write(`${item.id} … `);
      const raced = await withTimeout(
        probeOne(page, baseUrl, item),
        EXAMPLE_TIMEOUT_MS,
        {
          id: item.id,
          title: item.title,
          status: 'timeout',
          error: `探测超过 ${EXAMPLE_TIMEOUT_MS}ms（页面可能卡死）`,
        },
      );
      const row = raced.value;
      rows.push(row);
      console.log(row.status, row.error ? `- ${row.error.split('\n')[0]}` : '');
      if (raced.timedOut || row.status === 'timeout') {
        ({ browser, page } = await recycleBrowser(browser, chromium));
      }
    }
  } finally {
    await browser.close().catch(() => {});
  }

  const ok = rows.filter((r) => r.status === 'ok');
  const bad = rows.filter((r) => r.status !== 'ok');
  const report = { categories, ok, bad, rows };
  console.log('\n--- JSON ---');
  console.log(JSON.stringify(report, null, 2));
  if (outPath) {
    await writeFile(outPath, JSON.stringify(report, null, 2), 'utf8');
    console.log(`已写入 ${outPath}（ok ${ok.length} / 共 ${rows.length}）`);
  }
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
