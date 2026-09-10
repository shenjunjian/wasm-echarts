/**
 * 官网示例：和弦图 minAngle
 * https://echarts.apache.org/examples/zh/editor.html?c=chord-minAngle
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
  const myChart = echarts.init(canvas);
  window.addEventListener('resize', () => {
    if (myChart.isDisposed()) return;
    sizeCanvas(canvas);
    myChart.resize();
  });

  let option;
  /*
  title: Chord minAngle
  category: chord
  titleCN: 和弦图 minAngle
  difficulty: 1
  since: 6.0.0
  */
  option = {
    tooltip: {},
    legend: {},
    series: [
      {
        type: 'chord',
        label: { show: true },
        minAngle: 30,
        data: [
          { name: 'A' },
          { name: 'B' },
          { name: 'C' },
          { name: 'D' },
          { name: 'E' },
          { name: 'F' }
        ],
        links: [
          { source: 'A', target: 'B', value: 40 },
          { source: 'B', target: 'C', value: 20 },
          { source: 'E', target: 'A', value: 5 }
        ]
      }
    ]
  };
  if (option) {
    myChart.setOption(option);
  }
}

main().catch((error) => {
  showPreviewError(error);
  console.error(error);
});
