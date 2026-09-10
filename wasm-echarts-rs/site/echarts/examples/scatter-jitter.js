/**
 * 官网示例：带抖动的散点图
 * https://echarts.apache.org/examples/zh/editor.html?c=scatter-jitter
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
  title: Scatter with Jittering
  category: scatter
  titleCN: 带抖动的散点图
  difficulty: 3
  since: 6.0.0
  */
  const grid = {
    left: 80,
    right: 50
  };
  const width = myChart.getWidth() - grid.left - grid.right;
  const data = [];
  for (let day = 0; day < 7; ++day) {
    for (let i = 0; i < 1000; ++i) {
      const y = Math.tan(i) / 2 + 7;
      data.push([day, y, Math.random()]);
    }
  }
  option = {
    title: {
      text: 'Scatter with Jittering'
    },
    grid,
    xAxis: {
      type: 'category',
      jitter: (width / 7) * 0.8,
      data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun']
    },
    yAxis: {
      type: 'value',
      max: 10,
      min: 0
    },
    series: [
      {
        name: 'Sleeping Hours',
        type: 'scatter',
        data,
        colorBy: 'data',
        itemStyle: {
          opacity: 0.4
        }
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
