/**
 * 官网示例：极坐标双数值轴
 * https://echarts.apache.org/examples/zh/editor.html?c=line-polar
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
  title: Two Value-Axes in Polar
  category: line
  titleCN: 极坐标双数值轴
  difficulty: 10
  */
  const data = [];
  for (let i = 0; i <= 100; i++) {
    let theta = (i / 100) * 360;
    let r = 5 * (1 + Math.sin((theta / 180) * Math.PI));
    data.push([r, theta]);
  }
  option = {
    title: {
      text: 'Two Value-Axes in Polar'
    },
    legend: {
      data: ['line']
    },
    polar: {},
    tooltip: {
      trigger: 'axis',
      axisPointer: {
        type: 'cross'
      }
    },
    angleAxis: {
      type: 'value',
      startAngle: 0
    },
    radiusAxis: {},
    series: [
      {
        coordinateSystem: 'polar',
        name: 'line',
        type: 'line',
        data: data
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
