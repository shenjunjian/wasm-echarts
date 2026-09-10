/**
 * 官网示例：和弦图边的颜色
 * https://echarts.apache.org/examples/zh/editor.html?c=chord-lineStyle-color
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
  title: Chord lineStyle.color
  category: chord
  titleCN: 和弦图边的颜色
  difficulty: 3
  since: 6.0.0
  */
  function generateSeries(id, lineColor) {
    return {
      type: 'chord',
      label: { show: true },
      center: [((id * 2 + 1) / 6) * 100 + '%', '50%'],
      radius: ['28%', '32%'],
      lineStyle: {
        color: lineColor
      },
      data: [{ name: 'A' }, { name: 'B' }, { name: 'C' }, { name: 'D' }],
      links: [
        { source: 'A', target: 'B', value: 30 },
        { source: 'A', target: 'C', value: 20 },
        { source: 'B', target: 'D', value: 10 },
        { source: 'C', target: 'A', value: 15 },
        { source: 'D', target: 'A', value: 25 }
      ]
    };
  }
  function generateTitle(id, text) {
    return {
      text,
      left: ((id * 2 + 1) / 6) * 100 + '%',
      top: '25%',
      textAlign: 'center',
      padding: 0
    };
  }
  option = {
    tooltip: {},
    legend: {},
    series: [
      generateSeries(0, 'source'),
      generateSeries(1, 'target'),
      generateSeries(2, 'gradient')
    ],
    title: [
      {
        text: 'lineStyle.color',
        textStyle: {
          fontSize: 24
        }
      },
      generateTitle(0, 'source'),
      generateTitle(1, 'target'),
      generateTitle(2, 'gradient')
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
