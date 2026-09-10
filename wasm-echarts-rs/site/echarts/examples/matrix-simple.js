/**
 * 官网示例：简单的矩阵图
 * https://echarts.apache.org/examples/zh/editor.html?c=matrix-simple
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Simple Matrix
    category: matrix
    titleCN: 简单的矩阵图
    difficulty: 1
    since: 6.0.0
    */
    option = {
      matrix: {
        x: {
          data: [
            {
              value: 'A',
              children: [
                'A1',
                'A2',
                {
                  value: 'A3',
                  children: ['A31', 'A32']
                }
              ]
            }
          ]
        },
        y: {
          data: ['U', 'V']
        },
        top: 150,
        bottom: 150
      },
      visualMap: {
        type: 'continuous',
        min: 0,
        max: 80,
        top: 'middle',
        dimension: 2,
        calculable: true
      },
      series: {
        type: 'heatmap',
        coordinateSystem: 'matrix',
        data: [
          ['A1', 'U', 10],
          ['A1', 'V', 20],
          ['A2', 'U', 30],
          ['A2', 'V', 40],
          ['A31', 'U', 50],
          ['A3', 'V', 60]
        ],
        label: {
          show: true
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
