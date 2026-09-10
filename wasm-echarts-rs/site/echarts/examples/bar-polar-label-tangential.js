/**
 * 官网示例：极坐标柱状图标签
 * https://echarts.apache.org/examples/zh/editor.html?c=bar-polar-label-tangential
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
  title: Tangential Polar Bar Label Position
  titleCN: 极坐标柱状图标签
  category: bar
  difficulty: 2
  */
  option = {
    title: [
      {
        text: 'Tangential Polar Bar Label Position (middle)'
      }
    ],
    polar: {
      radius: [30, '80%']
    },
    angleAxis: {
      max: 4,
      startAngle: 75
    },
    radiusAxis: {
      type: 'category',
      data: ['a', 'b', 'c', 'd']
    },
    tooltip: {},
    series: {
      type: 'bar',
      data: [2, 1.2, 2.4, 3.6],
      coordinateSystem: 'polar',
      label: {
        show: true,
        position: 'middle',
        formatter: '{b}: {c}'
      }
    }
  };
  if (option) {
    myChart.setOption(option);
  }
}

main().catch((error) => {
  showPreviewError(error);
  console.error(error);
});
