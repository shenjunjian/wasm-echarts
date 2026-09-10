/**
 * 官网示例：相关矩阵（热力图）
 * https://echarts.apache.org/examples/zh/editor.html?c=matrix-correlation-heatmap
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
  title: Correlation Matrix (Heatmap)
  category: matrix
  titleCN: 相关矩阵（热力图）
  difficulty: 2
  since: 6.0.0
  */
  const xCnt = 8;
  const yCnt = xCnt;
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
      if (i >= j) {
        data.push(['X' + i, 'Y' + j, i === j ? 1 : Math.random() * 2 - 1]);
      }
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
      left: 'center'
    },
    series: {
      type: 'heatmap',
      coordinateSystem: 'matrix',
      data,
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
