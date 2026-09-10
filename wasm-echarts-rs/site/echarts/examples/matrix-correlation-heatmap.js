/**
 * 官网示例：相关矩阵（热力图）
 * https://echarts.apache.org/examples/zh/editor.html?c=matrix-correlation-heatmap
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
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
    return option;
  } catch (error) {
    if (option) {
      try {
        myChart.setOption(option);
      } catch {
        // 保留原始错误
      }
    }
    throw error;
  }
});
