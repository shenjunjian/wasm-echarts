/**
 * 官网示例：极坐标系 endAngle
 * https://echarts.apache.org/examples/zh/editor.html?c=polar-endAngle
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
  title: Polar endAngle
  category: bar
  titleCN: 极坐标系 endAngle
  difficulty: 2
  */
  option = {
    tooltip: {},
    angleAxis: [
      {
        type: 'category',
        polarIndex: 0,
        startAngle: 90,
        endAngle: 0,
        data: ['S1', 'S2', 'S3']
      },
      {
        type: 'category',
        polarIndex: 1,
        startAngle: -90,
        endAngle: -180,
        data: ['T1', 'T2', 'T3']
      }
    ],
    radiusAxis: [{ polarIndex: 0 }, { polarIndex: 1 }],
    polar: [{}, {}],
    series: [
      {
        type: 'bar',
        polarIndex: 0,
        data: [1, 2, 3],
        coordinateSystem: 'polar'
      },
      {
        type: 'bar',
        polarIndex: 1,
        data: [1, 2, 3],
        coordinateSystem: 'polar'
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
