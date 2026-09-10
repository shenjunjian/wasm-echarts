/**
 * 从 Apache ECharts 官网拉取折线图示例，包装成 wasm-echarts 画廊条目。
 * 不补齐未实现特性；运行时报错由 official-runtime 展示。
 */

import { mkdir, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const siteRoot = join(__dirname, '..');
const examplesDir = join(siteRoot, 'echarts', 'examples');
const publicOfficial = join(siteRoot, 'public', 'echarts-official');

const CHART_LIST_URL =
  'https://raw.githubusercontent.com/apache/echarts-examples/master/src/data/chart-list-data.js';
const EXAMPLE_JS_URL = (id) =>
  `https://echarts.apache.org/examples/examples/js/${id}.js`;
const ASSET_BASE = 'https://echarts.apache.org/examples';

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

function wrapOfficialSource(id, title, officialSource) {
  const body = normalizeOptionBinding(officialSource.trim());
  return `/**
 * 官网折线示例：${title}
 * https://echarts.apache.org/examples/zh/editor.html?c=${id}
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, $, app }) => {
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

async function main() {
  console.log('拉取 chart-list-data.js …');
  const listSource = await fetchText(CHART_LIST_URL);
  const all = parseChartList(listSource);
  const lineExamples = all.filter(
    (item) =>
      Array.isArray(item.category) &&
      item.category.includes('line') &&
      !String(item.id).startsWith('doc-example/'),
  );

  console.log(`折线类别示例 ${lineExamples.length} 个`);
  const catalog = [];
  const assetPaths = new Set();

  for (const item of lineExamples) {
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
    join(examplesDir, 'official-line-catalog.js'),
    `/** 由 scripts/sync-official-line-examples.mjs 生成，勿手改 */\nexport const officialLineExamples = ${JSON.stringify(catalog, null, 2)};\n`,
    'utf8',
  );

  console.log(`下载静态资源 ${assetPaths.size} 个 …`);
  for (const assetPath of [...assetPaths].sort()) {
    const url = ASSET_BASE + assetPath;
    const dest = join(publicOfficial, assetPath.replace(/^\//, ''));
    process.stdout.write(`  ${assetPath} … `);
    try {
      const buf = await fetchBuffer(url);
      await mkdir(dirname(dest), { recursive: true });
      await writeFile(dest, buf);
      console.log(`${buf.length} bytes`);
    } catch (err) {
      console.log(`FAILED ${err.message}`);
    }
  }

  console.log('done');
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
