import initWasm, { init, registerFont } from '@wasm-echarts';
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
  await loadFont();

  const canvas = document.getElementById('canvas');
  canvas.width = width;
  canvas.height = height;
  canvas.style.width = `${width}px`;
  canvas.style.height = `${height}px`;

  const chart = init(canvas);
  chart.setOption({
    series: [
      {
        type: 'pie',
        name: '占比',
        radius: ['30%', '55%'],
        center: ['50%', '50%'],
        label: { show: true, formatter: '{b}: {d}%' },
        data: [
          { name: '直接访问', value: 335 },
          { name: '邮件营销', value: 310 },
          { name: '联盟广告', value: 234 },
          { name: '视频广告', value: 135 },
          { name: '搜索引擎', value: 1548 },
        ],
      },
    ],
  });
}

main();
