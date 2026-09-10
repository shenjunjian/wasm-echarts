/**
 * 从 Apache ECharts 官网按图类拉取示例，包装成 wasm-echarts 画廊条目。
 * 生成脚本自己 init/setOption；official-env 只提供 ROOT_PATH / $ / app。
 * 不补齐未实现特性；运行时报错由 showPreviewError 展示。
 *
 * 用法：
 *   node scripts/sync-official-examples.mjs --category bar
 *   node scripts/sync-official-examples.mjs --category bar --category pie
 *   node scripts/sync-official-examples.mjs --list
 *   node scripts/sync-official-examples.mjs --rewrite-existing
 *   node scripts/sync-official-examples.mjs --category scatter --local-dir C:\\Users\\shen\\Desktop\\echarts-examples-gh-pages
 *
 * `--local-dir` / 环境变量 ECHARTS_EXAMPLES_DIR：apache/echarts-examples 克隆根目录。
 * `--rewrite-existing`：按当前模板重包已同步示例（不重新下载官网源码）。
 * 静态资源优先读 localDir/public；网络请求带超时，避免大文件卡住。
 */

import { mkdir, readdir, readFile, writeFile, stat } from 'node:fs/promises';
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
const FETCH_TIMEOUT_MS = 20000;

const KNOWN_CATEGORIES = new Set(OFFICIAL_CATEGORY_GROUPS.map((g) => g.category));

/**
 * 主线程会卡死的大数据示例：facade `useWorker`。
 * 仅收录 option 可结构化克隆的条目（无 formatter / renderItem 等函数）。
 */
export const WORKER_EXAMPLE_IDS = new Set([
  'bar-large',
  'scatter-large',
  'candlestick-large',
  'parallel-nutrients',
]);

export function parseArgs(argv) {
  const categories = [];
  let listOnly = false;
  let rewriteExisting = false;
  let localDir = process.env.ECHARTS_EXAMPLES_DIR || '';
  for (let i = 0; i < argv.length; i++) {
    const arg = argv[i];
    if (arg === '--list') {
      listOnly = true;
      continue;
    }
    if (arg === '--rewrite-existing') {
      rewriteExisting = true;
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
    if (arg === '--local-dir') {
      const value = argv[++i];
      if (!value) {
        throw new Error('--local-dir 需要 echarts-examples 仓库路径');
      }
      localDir = value;
      continue;
    }
    if (arg.startsWith('--local-dir=')) {
      localDir = arg.slice('--local-dir='.length);
      continue;
    }
    throw new Error(`未知参数: ${arg}`);
  }
  return {
    categories,
    listOnly,
    rewriteExisting,
    localDir: localDir ? resolve(localDir) : '',
  };
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
  const res = await fetch(url, { signal: AbortSignal.timeout(FETCH_TIMEOUT_MS) });
  if (!res.ok) {
    throw new Error(`GET ${url} failed: ${res.status}`);
  }
  return res.text();
}

async function fetchBuffer(url) {
  const res = await fetch(url, { signal: AbortSignal.timeout(FETCH_TIMEOUT_MS) });
  if (!res.ok) {
    throw new Error(`GET ${url} failed: ${res.status}`);
  }
  return Buffer.from(await res.arrayBuffer());
}

function localPublicFile(localDir, assetPath) {
  if (!localDir) return '';
  return join(localDir, 'public', assetPath.replace(/^\//, ''));
}

async function readLocalFile(filePath) {
  const buf = await readFile(filePath);
  if (!buf.length) {
    throw new Error(`local empty: ${filePath}`);
  }
  return buf;
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

export function wrapOfficialSource(id, title, officialSource) {
  const body = normalizeOptionBinding(officialSource.trim());
  const useWorker = WORKER_EXAMPLE_IDS.has(id);
  const initCall = useWorker
    ? 'echarts.init(canvas, null, { useWorker: true })'
    : 'echarts.init(canvas)';
  const setOptionCall = useWorker
    ? 'await myChart.setOption(option)'
    : 'myChart.setOption(option)';
  return `/**
 * 官网示例：${title}
 * https://echarts.apache.org/examples/zh/editor.html?c=${id}
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import initWasm, * as echarts from '@wasm-echarts';
import { ROOT_PATH, CDN_PATH, $, app, sizeCanvas, showPreviewError } from '../../src/echarts/official-env.js';
import { ensureDefaultFont } from '../../src/echarts/fonts.js';

async function main() {
  await initWasm();
  await ensureDefaultFont();

  const canvas = document.getElementById('canvas');
  if (!canvas) {
    throw new Error('缺少 #canvas');
  }
  sizeCanvas(canvas);
  const myChart = ${initCall};
  window.addEventListener('resize', () => {
    if (myChart.isDisposed()) return;
    sizeCanvas(canvas);
    myChart.resize();
  });

  let option;
${indent(body, 2)}
  if (option) {
    ${setOptionCall};
  }
}

main().catch((error) => {
  showPreviewError(error);
  console.error(error);
});
`;
}

const WRAPPED_TRY_START = '  try {\n';
const WRAPPED_RETURN = '\n    return option;';

export function extractWrappedOfficial(source, fileName = '') {
  const text = source.replace(/\r\n/g, '\n');
  const idFromFile = fileName.replace(/\.js$/i, '');
  const idMatch = text.match(/editor\.html\?c=([^\s*]+)/);
  const titleMatch = text.match(/官网[^\n：]*：([^\n*]+)/);
  const start = text.indexOf(WRAPPED_TRY_START);
  const end = text.lastIndexOf(WRAPPED_RETURN);
  if (start < 0 || end < 0 || end <= start) {
    throw new Error(`无法从 ${fileName || 'source'} 抽出官网正文`);
  }
  const body = text.slice(start + WRAPPED_TRY_START.length, end).replace(/^ {4}/gm, '');
  return {
    id: (idMatch && idMatch[1]) || idFromFile,
    title: ((titleMatch && titleMatch[1]) || idFromFile).trim(),
    body,
  };
}

async function rewriteExistingExamples() {
  const names = await readdir(examplesDir);
  let count = 0;
  for (const name of names.sort()) {
    if (!name.endsWith('.js') || name.startsWith('official-')) continue;
    const filePath = join(examplesDir, name);
    const source = await readFile(filePath, 'utf8');
    if (!source.includes('runOfficialExample')) continue;
    const { id, title, body } = extractWrappedOfficial(source, name);
    await writeFile(filePath, wrapOfficialSource(id, title, body), 'utf8');
    count += 1;
    console.log(`  ${name}`);
  }
  console.log(`rewrote ${count} examples`);
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
  node scripts/sync-official-examples.mjs --rewrite-existing
  node scripts/sync-official-examples.mjs --category scatter --local-dir <echarts-examples 根目录>

已知类别: ${known}`);
}

async function downloadAssets(assetPaths, localDir) {
  console.log(`下载静态资源 ${assetPaths.size} 个 …`);
  if (localDir) {
    console.log(`本地优先: ${localDir}`);
  }
  for (const assetPath of [...assetPaths].sort()) {
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
      let buf;
      let source = 'network';
      const localFile = localPublicFile(localDir, assetPath);
      if (localFile) {
        try {
          buf = await readLocalFile(localFile);
          source = 'local';
        } catch {
          buf = undefined;
        }
      }
      if (!buf) {
        buf = await fetchBuffer(ASSET_BASE + assetPath);
      }
      await mkdir(dirname(dest), { recursive: true });
      await writeFile(dest, buf);
      console.log(`${buf.length} bytes (${source})`);
    } catch (err) {
      console.log(`FAILED ${err.message}`);
    }
  }
}

export async function syncCategories(categories, { listOnly = false, localDir = '' } = {}) {
  console.log('拉取 chart-list-data.js …');
  let listSource;
  const localList = localDir ? join(localDir, 'src', 'data', 'chart-list-data.js') : '';
  if (localList) {
    try {
      listSource = (await readFile(localList, 'utf8'));
      console.log(`chart-list-data.js 来自本地 ${localList}`);
    } catch {
      listSource = undefined;
    }
  }
  if (!listSource) {
    listSource = await fetchText(CHART_LIST_URL);
  }
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
      let officialSource;
      const localJs = localDir ? join(localDir, 'public', 'examples', 'js', `${id}.js`) : '';
      if (localJs) {
        try {
          officialSource = await readFile(localJs, 'utf8');
        } catch {
          officialSource = undefined;
        }
      }
      if (!officialSource) {
        officialSource = await fetchText(EXAMPLE_JS_URL(id));
      }
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
    await downloadAssets(assetPaths, localDir);
  }
  console.log('done');
}

export async function main(argv = process.argv.slice(2)) {
  const { categories, listOnly, rewriteExisting, localDir } = parseArgs(argv);
  if (rewriteExisting) {
    await rewriteExistingExamples();
    return;
  }
  if (!listOnly && categories.length === 0) {
    printUsage();
    process.exitCode = 1;
    return;
  }
  await syncCategories(categories, { listOnly, localDir });
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
