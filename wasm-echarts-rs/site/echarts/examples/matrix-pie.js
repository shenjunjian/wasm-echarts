/**
 * 官网示例：矩阵布局下的饼图
 * https://echarts.apache.org/examples/zh/editor.html?c=matrix-pie
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
  title: Pie Charts in Matrix
  category: matrix
  titleCN: 矩阵布局下的饼图
  difficulty: 2
  since: 6.0.0
  */
  const xCnt = 9;
  const yCnt = 6;
  const series = [];
  for (let i = 0; i < xCnt; ++i) {
    for (let j = 0; j < yCnt; ++j) {
      series.push({
        type: 'pie',
        coordinateSystem: 'matrix',
        center: [`Grade ${i + 1}`, `Class ${j + 1}`],
        radius: 18,
        data: [
          {
            value: Math.round(Math.random() * 10) + 10,
            name: 'Male'
          },
          {
            value: Math.round(Math.random() * 10) + 10,
            name: 'Female'
          }
        ],
        label: {
          show: false
        },
        emphasis: {
          label: {
            show: false
          }
        }
      });
    }
  }
  option = {
    legend: {
      show: true,
      bottom: 40
    },
    matrix: {
      x: {
        data: [
          {
            value: 'Primary School',
            children: Array.from({ length: 5 }, (_, i) => {
              return `Grade ${i + 1}`;
            })
          },
          {
            value: 'High School',
            children: Array.from({ length: 4 }, (_, i) => {
              return `Grade ${i + 6}`;
            })
          }
        ]
      },
      y: {
        data: Array.from({ length: 6 }, (_, i) => {
          return `Class ${i + 1}`;
        })
      },
      top: 80,
      bottom: 80
    },
    series,
    tooltip: {
      show: true
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
