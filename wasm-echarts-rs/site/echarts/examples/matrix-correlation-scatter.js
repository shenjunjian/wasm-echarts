/**
 * 官网示例：相关矩阵（散点图）
 * https://echarts.apache.org/examples/zh/editor.html?c=matrix-correlation-scatter
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
  title: Correlation Matrix (Scatter)
  category: matrix
  titleCN: 相关矩阵（散点图）
  difficulty: 2
  since: 6.0.0
  */
  const xCnt = 10;
  const yCnt = 6;
  const xData = [];
  const yData = [];
  for (let i = 0; i < xCnt; ++i) {
    xData.push({
      value: 'X' + (i + 1)
    });
  }
  for (let i = 0; i < yCnt; ++i) {
    yData.push({
      value: 'Y' + (i + 1)
    });
  }
  const data = [];
  for (let i = 1; i <= xCnt; ++i) {
    for (let j = 1; j <= yCnt; ++j) {
      data.push(['X' + i, 'Y' + j, Math.random() * 2 - 1]);
    }
  }
  option = {
    matrix: {
      x: {
        data: xData
      },
      y: {
        data: yData
      },
      top: 80
    },
    visualMap: {
      type: 'continuous',
      min: -1,
      max: 1,
      dimension: 2,
      calculable: true,
      orient: 'horizontal',
      top: 5,
      left: 'center',
      inRange: {
        color: [
          '#313695',
          '#4575b4',
          '#74add1',
          '#abd9e9',
          '#e0f3f8',
          '#ffffbf',
          '#fee090',
          '#fdae61',
          '#f46d43',
          '#d73027',
          '#a50026'
        ],
        symbolSize: [15, 40]
      }
    },
    series: {
      type: 'scatter',
      coordinateSystem: 'matrix',
      data,
      itemStyle: {
        opacity: 1
      },
      label: {
        show: true,
        formatter: (params) => params.value[2].toFixed(2)
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
