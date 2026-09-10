/**
 * 官网示例：自定义单个柱子颜色
 * https://echarts.apache.org/examples/zh/editor.html?c=bar-data-color
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Set Style of Single Bar.
    category: bar
    titleCN: 自定义单个柱子颜色
    difficulty: 1
    */
    option = {
      xAxis: {
        type: 'category',
        data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun']
      },
      yAxis: {
        type: 'value'
      },
      series: [
        {
          data: [
            120,
            {
              value: 200,
              itemStyle: {
                color: '#505372'
              }
            },
            150,
            80,
            70,
            110,
            130
          ],
          type: 'bar'
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
