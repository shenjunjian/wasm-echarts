/**
 * 官网示例：柱状图排序
 * https://echarts.apache.org/examples/zh/editor.html?c=data-transform-sort-bar
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Sort Data in Bar Chart
    category: dataset, bar, transform
    titleCN: 柱状图排序
    difficulty: 0
    */
    option = {
      dataset: [
        {
          dimensions: ['name', 'age', 'profession', 'score', 'date'],
          source: [
            ['Hannah Krause', 41, 'Engineer', 314, '2011-02-12'],
            ['Zhao Qian', 20, 'Teacher', 351, '2011-03-01'],
            ['Jasmin Krause ', 52, 'Musician', 287, '2011-02-14'],
            ['Li Lei', 37, 'Teacher', 219, '2011-02-18'],
            ['Karle Neumann', 25, 'Engineer', 253, '2011-04-02'],
            ['Adrian Groß', 19, 'Teacher', '-', '2011-01-16'],
            ['Mia Neumann', 71, 'Engineer', 165, '2011-03-19'],
            ['Böhm Fuchs', 36, 'Musician', 318, '2011-02-24'],
            ['Han Meimei', 67, 'Engineer', 366, '2011-03-12']
          ]
        },
        {
          transform: {
            type: 'sort',
            config: { dimension: 'score', order: 'desc' }
          }
        }
      ],
      xAxis: {
        type: 'category',
        axisLabel: { interval: 0, rotate: 30 }
      },
      yAxis: {},
      series: {
        type: 'bar',
        encode: { x: 'name', y: 'score' },
        datasetIndex: 1
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
