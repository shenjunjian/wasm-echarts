/**
 * 官网示例：双数值轴折线图
 * https://echarts.apache.org/examples/zh/editor.html?c=line-in-cartesian-coordinate-system
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
  title: Line Chart in Cartesian Coordinate System
  category: line
  titleCN: 双数值轴折线图
  difficulty: 7
  */
  option = {
    xAxis: {},
    yAxis: {},
    series: [
      {
        data: [
          [10, 40],
          [50, 100],
          [40, 20]
        ],
        type: 'line'
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
