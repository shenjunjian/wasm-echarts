/**
 * 官网示例：系列按行和按列排布
 * https://echarts.apache.org/examples/zh/editor.html?c=dataset-series-layout-by
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Series Layout By Column or Row
    category: 'dataset, bar'
    titleCN: 系列按行和按列排布
    difficulty: 5
    */
    option = {
      legend: {},
      tooltip: {},
      dataset: {
        source: [
          ['product', '2012', '2013', '2014', '2015'],
          ['Matcha Latte', 41.1, 30.4, 65.1, 53.3],
          ['Milk Tea', 86.5, 92.1, 85.7, 83.1],
          ['Cheese Cocoa', 24.1, 67.2, 79.5, 86.4]
        ]
      },
      xAxis: [
        { type: 'category', gridIndex: 0 },
        { type: 'category', gridIndex: 1 }
      ],
      yAxis: [{ gridIndex: 0 }, { gridIndex: 1 }],
      grid: [{ bottom: '55%' }, { top: '55%' }],
      series: [
        // These series are in the first grid.
        { type: 'bar', seriesLayoutBy: 'row' },
        { type: 'bar', seriesLayoutBy: 'row' },
        { type: 'bar', seriesLayoutBy: 'row' },
        // These series are in the second grid.
        { type: 'bar', xAxisIndex: 1, yAxisIndex: 1 },
        { type: 'bar', xAxisIndex: 1, yAxisIndex: 1 },
        { type: 'bar', xAxisIndex: 1, yAxisIndex: 1 },
        { type: 'bar', xAxisIndex: 1, yAxisIndex: 1 }
      ]
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
