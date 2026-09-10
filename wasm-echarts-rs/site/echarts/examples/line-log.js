/**
 * 官网折线示例：对数轴示例
 * https://echarts.apache.org/examples/zh/editor.html?c=line-log
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Log Axis
    category: line
    titleCN: 对数轴示例
    difficulty: 7
    */
    option = {
      title: {
        text: 'Log Axis',
        left: 'center'
      },
      tooltip: {
        trigger: 'item',
        formatter: '{a} <br/>{b} : {c}'
      },
      legend: {
        left: 'left'
      },
      xAxis: {
        type: 'category',
        name: 'x',
        splitLine: { show: false },
        data: ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I']
      },
      grid: {
        left: '3%',
        right: '4%',
        bottom: '3%',
        containLabel: true
      },
      yAxis: {
        type: 'log',
        name: 'y',
        minorSplitLine: {
          show: true
        }
      },
      series: [
        {
          name: 'Log2',
          type: 'line',
          data: [1, 3, 9, 27, 81, 247, 741, 2223, 6669]
        },
        {
          name: 'Log3',
          type: 'line',
          data: [1, 2, 4, 8, 16, 32, 64, 128, 256]
        },
        {
          name: 'Log1/2',
          type: 'line',
          data: [
            1 / 2,
            1 / 4,
            1 / 8,
            1 / 16,
            1 / 32,
            1 / 64,
            1 / 128,
            1 / 256,
            1 / 512
          ]
        }
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
