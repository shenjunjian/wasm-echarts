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
    xAxis: { type: 'value', scale: true },
    yAxis: { type: 'value', scale: true },
    series: [
      {
        type: 'scatter',
        name: '样本',
        symbol: 'circle',
        symbolSize: 12,
        data: [
          [10.0, 8.04],
          [8.07, 6.95],
          [13.0, 7.58],
          [9.05, 8.81],
          [11.0, 8.33],
          [14.0, 7.66],
          [13.4, 6.81],
          [10.0, 6.33],
          [14.0, 8.96],
          [12.5, 6.82],
        ],
      },
    ],
  });
}

main();
