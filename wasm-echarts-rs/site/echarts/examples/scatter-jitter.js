/**
 * 官网示例：带抖动的散点图
 * https://echarts.apache.org/examples/zh/editor.html?c=scatter-jitter
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Scatter with Jittering
    category: scatter
    titleCN: 带抖动的散点图
    difficulty: 3
    since: 6.0.0
    */
    const grid = {
      left: 80,
      right: 50
    };
    const width = myChart.getWidth() - grid.left - grid.right;
    const data = [];
    for (let day = 0; day < 7; ++day) {
      for (let i = 0; i < 1000; ++i) {
        const y = Math.tan(i) / 2 + 7;
        data.push([day, y, Math.random()]);
      }
    }
    option = {
      title: {
        text: 'Scatter with Jittering'
      },
      grid,
      xAxis: {
        type: 'category',
        jitter: (width / 7) * 0.8,
        data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun']
      },
      yAxis: {
        type: 'value',
        max: 10,
        min: 0
      },
      series: [
        {
          name: 'Sleeping Hours',
          type: 'scatter',
          data,
          colorBy: 'data',
          itemStyle: {
            opacity: 0.4
          }
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
