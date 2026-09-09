import initWasm, { init, registerFont } from '@wasm-echarts';

const width = 480;
const height = 360;
const FONT_FAMILY = 'Noto Sans SC';
const FONT_URL = '/fonts/NotoSansSC-Regular.ttf';

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

async function main() {
  await initWasm();
  await loadFont();

  const canvas = document.getElementById('canvas');
  canvas.width = width;
  canvas.height = height;
  canvas.style.width = `${width}px`;
  canvas.style.height = `${height}px`;

  const chart = init(canvas);
  chart.setOption({
    xAxis: { type: 'category', data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri'] },
    yAxis: { type: 'value' },
    series: [{ type: 'line', name: '销量', data: [120, 200, 150, 80, 70] }],
  });

  const avgMs = chart.benchmarkRender(30);
  const logEl = document.getElementById('log');
  if (logEl) {
    logEl.textContent = `benchmarkRender 均值 ${avgMs.toFixed(2)} ms（30 次）`;
  }
}

main();
