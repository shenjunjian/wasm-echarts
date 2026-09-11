import initWasm, { init, dispose, registerFont } from '@wasm-echarts';
import { DEFAULT_FONT_URL as FONT_URL } from '../../src/shared/site-base.js';

const width = 480;
const height = 360;
const FONT_FAMILY = 'Noto Sans SC';

async function loadFont() {
  const response = await fetch(FONT_URL);
  if (!response.ok) {
    throw new Error(`字体加载失败: ${FONT_URL} (${response.status})`);
  }
  const bytes = new Uint8Array(await response.arrayBuffer());
  registerFont(bytes, {
    familyName: FONT_FAMILY,
    sansSerif: [FONT_FAMILY],
  });
}

function bindCanvas(id) {
  const canvas = document.getElementById(id);
  canvas.width = width;
  canvas.height = height;
  canvas.style.width = `${width}px`;
  canvas.style.height = `${height}px`;
  return canvas;
}

async function main() {
  await initWasm();
  await loadFont();

  const mergeCanvas = bindCanvas('canvas-merge');
  const replaceCanvas = bindCanvas('canvas-not-merge');

  const merged = init(mergeCanvas);
  merged.setOption({
    xAxis: { type: 'category', data: ['A', 'B', 'C'] },
    yAxis: { type: 'value' },
    series: [{ type: 'bar', name: '第一批', data: [10, 20, 30] }],
  });
  merged.setOption({
    series: [{ type: 'line', name: '第二批', data: [15, 25, 18] }],
  });
  const mergedOption = merged.getOption();
  if (!mergedOption.series || mergedOption.series[0].type !== 'line') {
    throw new Error('getOption 未返回合并后的 series');
  }

  let replaced = init(replaceCanvas);
  replaced.setOption({
    xAxis: { type: 'category', data: ['A', 'B', 'C'] },
    yAxis: { type: 'value' },
    series: [{ type: 'bar', name: '第一批', data: [10, 20, 30] }],
  });
  dispose(replaced);
  replaced = init(replaceCanvas);
  replaced.setOption({
    xAxis: { type: 'category', data: ['A', 'B', 'C'] },
    yAxis: { type: 'value' },
    series: [{ type: 'bar', name: '第一批', data: [10, 20, 30] }],
  });
  replaced.setOption({
    series: [{ type: 'bar', name: '替换', data: [1, 2, 3] }],
  }, true);

  const logEl = document.getElementById('log');
  if (logEl) {
    logEl.textContent = `深合并 series[0].type=${mergedOption.series[0].type}；右侧 notMerge + dispose/init 已跑完`;
  }
}

main();
