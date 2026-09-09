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
    xAxis: {
      type: 'category',
      data: ['A', 'B', 'C', 'D'],
      axisLabel: { formatter: '{value}' },
    },
    yAxis: { type: 'value', axisLabel: { formatter: '{value}' } },
    series: [
      {
        type: 'bar',
        name: '数量',
        data: [40, 90, 60, 120],
        label: { show: true },
      },
    ],
  });
}

main();
