/**
 * 官网示例：指数回归（使用统计插件）
 * https://echarts.apache.org/examples/zh/editor.html?c=scatter-exponential-regression
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Exponential Regression
    category: scatter
    titleCN: 指数回归（使用统计插件）
    difficulty: 1
    */
    // See https://github.com/ecomfe/echarts-stat
    echarts.registerTransform(ecStat.transform.regression);
    option = {
      dataset: [
        {
          source: [
            [1, 4862.4],
            [2, 5294.7],
            [3, 5934.5],
            [4, 7171.0],
            [5, 8964.4],
            [6, 10202.2],
            [7, 11962.5],
            [8, 14928.3],
            [9, 16909.2],
            [10, 18547.9],
            [11, 21617.8],
            [12, 26638.1],
            [13, 34634.4],
            [14, 46759.4],
            [15, 58478.1],
            [16, 67884.6],
            [17, 74462.6],
            [18, 79395.7]
          ]
        },
        {
          transform: {
            type: 'ecStat:regression',
            config: {
              method: 'exponential'
              // 'end' by default
              // formulaOn: 'start'
            }
          }
        }
      ],
      title: {
        text: '1981 - 1998 gross domestic product GDP (trillion yuan)',
        subtext: 'By ecStat.regression',
        sublink: 'https://github.com/ecomfe/echarts-stat',
        left: 'center'
      },
      tooltip: {
        trigger: 'axis',
        axisPointer: {
          type: 'cross'
        }
      },
      xAxis: {
        splitLine: {
          lineStyle: {
            type: 'dashed'
          }
        }
      },
      yAxis: {
        splitLine: {
          lineStyle: {
            type: 'dashed'
          }
        }
      },
      series: [
        {
          name: 'scatter',
          type: 'scatter',
          datasetIndex: 0
        },
        {
          name: 'line',
          type: 'line',
          smooth: true,
          datasetIndex: 1,
          symbolSize: 0.1,
          symbol: 'circle',
          label: { show: true, fontSize: 16 },
          labelLayout: { dx: -20 },
          encode: { label: 2, tooltip: 1 }
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
