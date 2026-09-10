/**
 * 从 Apache ECharts 官网按图类拉取示例，包装成 wasm-echarts 画廊条目。
 * 不补齐未实现特性；运行时报错由 official-runtime 展示。
 *
 * 用法：
 *   node scripts/sync-official-examples.mjs --category bar
 *   node scripts/sync-official-examples.mjs --category bar --category pie
 *   node scripts/sync-official-examples.mjs --list
 */

import { mkdir, writeFile, stat } from 'node:fs/promises';
import { dirname, join, normalize, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { OFFICIAL_CATEGORY_GROUPS } from '../src/echarts/official-gallery-meta.js';

const __dirname = dirname(fileURLToPath(import.meta.url));
const siteRoot = join(__dirname, '..');
const examplesDir = join(siteRoot, 'echarts', 'examples');
const publicOfficial = join(siteRoot, 'public', 'echarts-official');

const CHART_LIST_URL =
  'https://raw.githubusercontent.com/apache/echarts-examples/master/src/data/chart-list-data.js';
const EXAMPLE_JS_URL = (id) =>
  `https://echarts.apache.org/examples/examples/js/${id}.js`;
const ASSET_BASE = 'https://echarts.apache.org/examples';

const KNOWN_CATEGORIES = new Set(OFFICIAL_CATEGORY_GROUPS.map((g) => g.category));

export function parseArgs(argv) {
  const categories = [];
  let listOnly = false;
  for (let i = 0; i < argv.length; i++) {
    const arg = argv[i];
    if (arg === '--list') {
      listOnly = true;
      continue;
    }
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
    throw new Error(`未知参数: ${arg}`);
  }
  return { categories, listOnly };
}

const htmlTemplate = (id, title) => `<!DOCTYPE html>
<html lang="zh-CN">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>${escapeHtml(title)} · wasm-echarts</title>
  <link rel="stylesheet" href="/src/shared/preview.css" />
</head>
<body>
  <div class="preview-root preview-root--fill">
    <canvas id="canvas"></canvas>
  </div>
  <script type="module" src="./${id}.js"></script>
</body>
</html>
`;

function escapeHtml(value) {
  return String(value)
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;');
}

function indent(source, spaces = 2) {
  const pad = ' '.repeat(spaces);
  return source
    .split('\n')
    .map((line) => (line.length ? pad + line : line))
    .join('\n');
}

function normalizeOptionBinding(source) {
  let next = source.replace(/\b(?:const|let|var)\s+option\s*=/g, 'option =');
  next = next.replace(/\b(?:const|let|var)\s+option\s*;/g, '');
  return next;
}

function collectAssetPaths(source) {
  const paths = new Set();
  const re = /ROOT_PATH\s*\+\s*(['"])(\/[^'"]+)\1/g;
  let match;
  while ((match = re.exec(source))) {
    paths.add(match[2]);
  }
  return [...paths];
}

async function fetchText(url) {
  const res = await fetch(url);
  if (!res.ok) {
    throw new Error(`GET ${url} failed: ${res.status}`);
  }
  return res.text();
}

async function fetchBuffer(url) {
  const res = await fetch(url);
  if (!res.ok) {
    throw new Error(`GET ${url} failed: ${res.status}`);
  }
  return Buffer.from(await res.arrayBuffer());
}

function parseChartList(jsSource) {
  const start = jsSource.indexOf('[');
  const end = jsSource.lastIndexOf(']');
  if (start < 0 || end < 0) {
    throw new Error('chart-list-data.js 未找到数组');
  }
  return JSON.parse(jsSource.slice(start, end + 1));
}

function filterExamples(all, category) {
  return all.filter(
    (item) =>
      Array.isArray(item.category) &&
      item.category.includes(category) &&
      !String(item.id).startsWith('doc-example/'),
  );
}

function wrapOfficialSource(id, title, officialSource) {
  const body = normalizeOptionBinding(officialSource.trim());
  return `/**
 * 官网示例：${title}
 * https://echarts.apache.org/examples/zh/editor.html?c=${id}
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
${indent(body, 4)}
    return option;
  } catch (error) {
    if (option) {
      try {
        myChart.setOption(option);
      } catch {
        // 保留原始错误
      }
    }
    throw error;
  }
});
`;
}

function catalogSource(category, catalog) {
  return `/** 由 scripts/sync-official-examples.mjs --category ${category} 生成，勿手改 */
export const officialCategory = ${JSON.stringify(category)};
export const officialExamples = ${JSON.stringify(catalog, null, 2)};
`;
}

function printUsage() {
  const known = OFFICIAL_CATEGORY_GROUPS.map((g) => g.category).join(', ');
  console.error(`用法:
  node scripts/sync-official-examples.mjs --category <name> [--category <name> ...]
  node scripts/sync-official-examples.mjs --list

已知类别: ${known}`);
}

async function downloadAssets(assetPaths) {
  console.log(`下载静态资源 ${assetPaths.size} 个 …`);
  for (const assetPath of [...assetPaths].sort()) {
    const url = ASSET_BASE + assetPath;
    const dest = join(publicOfficial, assetPath.replace(/^\//, ''));
    process.stdout.write(`  ${assetPath} … `);
    try {
      try {
        const existing = await stat(dest);
        if (existing.isFile() && existing.size > 0) {
          console.log(`skip (${existing.size} bytes)`);
          continue;
        }
      } catch {
        // 不存在则下载
      }
      const buf = await fetchBuffer(url);
      await mkdir(dirname(dest), { recursive: true });
      await writeFile(dest, buf);
      console.log(`${buf.length} bytes`);
    } catch (err) {
      console.log(`FAILED ${err.message}`);
    }
  }
}

export async function syncCategories(categories, { listOnly = false } = {}) {
  console.log('拉取 chart-list-data.js …');
  const listSource = await fetchText(CHART_LIST_URL);
  const all = parseChartList(listSource);

  if (listOnly) {
    const counts = new Map();
    for (const item of all) {
      if (String(item.id).startsWith('doc-example/')) continue;
      for (const cat of item.category || []) {
        counts.set(cat, (counts.get(cat) || 0) + 1);
      }
    }
    for (const group of OFFICIAL_CATEGORY_GROUPS) {
      console.log(`${String(counts.get(group.category) || 0).padStart(4)}  ${group.category}  ${group.title}`);
    }
    const extras = [...counts.keys()].filter((cat) => !KNOWN_CATEGORIES.has(cat)).sort();
    if (extras.length) {
      console.log('\n未进画廊分组的类别:');
      for (const cat of extras) {
        console.log(`${String(counts.get(cat)).padStart(4)}  ${cat}`);
      }
    }
    return;
  }

  const assetPaths = new Set();
  for (const category of categories) {
    if (!KNOWN_CATEGORIES.has(category)) {
      throw new Error(`未知类别: ${category}`);
    }
    const items = filterExamples(all, category);
    console.log(`${category} 类别示例 ${items.length} 个`);
    const catalog = [];
    for (const item of items) {
      const id = item.id;
      const title = item.titleCN || item.title || id;
      process.stdout.write(`  ${id} … `);
      const officialSource = await fetchText(EXAMPLE_JS_URL(id));
      for (const path of collectAssetPaths(officialSource)) {
        assetPaths.add(path);
      }
      const wrapped = wrapOfficialSource(id, title, officialSource);
      await writeFile(join(examplesDir, `${id}.js`), wrapped, 'utf8');
      await writeFile(join(examplesDir, `${id}.html`), htmlTemplate(id, title), 'utf8');
      catalog.push({
        id,
        title,
        description: item.title || '',
        difficulty: item.difficulty ?? 0,
        category: item.category,
      });
      console.log('ok');
    }
    await writeFile(
      join(examplesDir, `official-${category}-catalog.js`),
      catalogSource(category, catalog),
      'utf8',
    );
  }

  if (assetPaths.size) {
    await downloadAssets(assetPaths);
  }
  console.log('done');
}

export async function main(argv = process.argv.slice(2)) {
  const { categories, listOnly } = parseArgs(argv);
  if (!listOnly && categories.length === 0) {
    printUsage();
    process.exitCode = 1;
    return;
  }
  await syncCategories(categories, { listOnly });
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
