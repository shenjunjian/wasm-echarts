/**
 * 官网示例：基础平行坐标
 * https://echarts.apache.org/examples/zh/editor.html?c=parallel-simple
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Basic Parallel
    category: parallel
    titleCN: 基础平行坐标
    difficulty: 1
    */
    option = {
      parallelAxis: [
        { dim: 0, name: 'Price' },
        { dim: 1, name: 'Net Weight' },
        { dim: 2, name: 'Amount' },
        {
          dim: 3,
          name: 'Score',
          type: 'category',
          data: ['Excellent', 'Good', 'OK', 'Bad']
        }
      ],
      series: {
        type: 'parallel',
        lineStyle: {
          width: 4
        },
        data: [
          [12.99, 100, 82, 'Good'],
          [9.99, 80, 77, 'OK'],
          [20, 120, 60, 'Excellent']
        ]
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
