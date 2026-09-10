/**
 * 官网示例：对数轴示例
 * https://echarts.apache.org/examples/zh/editor.html?c=line-log
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
  title: Log Axis
  category: line
  titleCN: 对数轴示例
  difficulty: 7
  */
  option = {
    title: {
      text: 'Log Axis',
      left: 'center'
    },
    tooltip: {
      trigger: 'item',
      formatter: '{a} <br/>{b} : {c}'
    },
    legend: {
      left: 'left'
    },
    xAxis: {
      type: 'category',
      name: 'x',
      splitLine: { show: false },
      data: ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I']
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      containLabel: true
    },
    yAxis: {
      type: 'log',
      name: 'y',
      minorSplitLine: {
        show: true
      }
    },
    series: [
      {
        name: 'Log2',
        type: 'line',
        data: [1, 3, 9, 27, 81, 247, 741, 2223, 6669]
      },
      {
        name: 'Log3',
        type: 'line',
        data: [1, 2, 4, 8, 16, 32, 64, 128, 256]
      },
      {
        name: 'Log1/2',
        type: 'line',
        data: [
          1 / 2,
          1 / 4,
          1 / 8,
          1 / 16,
          1 / 32,
          1 / 64,
          1 / 128,
          1 / 256,
          1 / 512
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
