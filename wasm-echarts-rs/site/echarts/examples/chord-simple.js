/**
 * 官网示例：基础和弦图
 * https://echarts.apache.org/examples/zh/editor.html?c=chord-simple
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
  title: Basic Chord
  category: chord
  titleCN: 基础和弦图
  difficulty: 0
  since: 6.0.0
  */
  option = {
    tooltip: {},
    legend: {},
    series: [
      {
        type: 'chord',
        clockwise: false,
        label: { show: true },
        lineStyle: { color: 'target' },
        data: [{ name: 'A' }, { name: 'B' }, { name: 'C' }, { name: 'D' }],
        links: [
          { source: 'A', target: 'B', value: 40 },
          { source: 'A', target: 'C', value: 20 },
          { source: 'B', target: 'D', value: 20 }
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
