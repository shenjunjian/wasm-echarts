/**
 * 官网示例：圆角环形图
 * https://echarts.apache.org/examples/zh/editor.html?c=polar-roundCap
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
  title: Rounded Bar on Polar
  category: bar
  titleCN: 圆角环形图
  difficulty: 7
  */
  option = {
    angleAxis: {
      max: 2,
      startAngle: 30,
      splitLine: {
        show: false
      }
    },
    radiusAxis: {
      type: 'category',
      data: ['v', 'w', 'x', 'y', 'z'],
      z: 10
    },
    polar: {},
    series: [
      {
        type: 'bar',
        data: [4, 3, 2, 1, 0],
        coordinateSystem: 'polar',
        name: 'Without Round Cap',
        itemStyle: {
          borderColor: 'red',
          opacity: 0.8,
          borderWidth: 1
        }
      },
      {
        type: 'bar',
        data: [4, 3, 2, 1, 0],
        coordinateSystem: 'polar',
        name: 'With Round Cap',
        roundCap: true,
        itemStyle: {
          borderColor: 'green',
          opacity: 0.8,
          borderWidth: 1
        }
      }
    ],
    legend: {
      show: true,
      data: ['Without Round Cap', 'With Round Cap']
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
