import initWasm, { init, use, registerFont } from '@wasm-echarts';
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

async function main() {
  await initWasm();
  use();
  await loadFont();

  const canvas = document.getElementById('canvas');
  canvas.width = width;
  canvas.height = height;
  canvas.style.width = `${width}px`;
  canvas.style.height = `${height}px`;

  const chart = init(canvas);
  chart.setOption({
    tooltip: {
      trigger: 'item',
      axisPointer: { type: 'line' },
      formatter(params) {
        return `${params.seriesName}<br/>${params.name}: ${params.value}`;
      },
    },
    dataZoom: [{ type: 'inside', xAxisIndex: 0 }],
    xAxis: { type: 'category', data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri'] },
    yAxis: { type: 'value' },
    series: [{ type: 'line', name: '销量', data: [120, 200, 150, 80, 70] }],
  });

  const logEl = document.getElementById('log');
  chart.on('click', (params) => {
    chart.dispatchAction({
      type: 'toggleSelect',
      seriesIndex: params.seriesIndex,
      dataIndex: params.dataIndex,
    });
    if (logEl) {
      logEl.textContent = `click select → seriesIndex=${params.seriesIndex}, dataIndex=${params.dataIndex}`;
    }
  });
}

main();
