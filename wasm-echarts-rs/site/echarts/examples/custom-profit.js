/**
 * 官网示例：利润分布直方图
 * https://echarts.apache.org/examples/zh/editor.html?c=custom-profit
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Profit
    category: custom
    titleCN: 利润分布直方图
    difficulty: 1
    */
    const colorList = [
      '#4f81bd',
      '#c0504d',
      '#9bbb59',
      '#604a7b',
      '#948a54',
      '#e46c0b'
    ];
    const data = [
      [10, 16, 3, 'A'],
      [16, 18, 15, 'B'],
      [18, 26, 12, 'C'],
      [26, 32, 22, 'D'],
      [32, 56, 7, 'E'],
      [56, 62, 17, 'F']
    ].map(function (item, index) {
      return {
        value: item,
        itemStyle: {
          color: colorList[index]
        }
      };
    });
    option = {
      title: {
        text: 'Profit',
        left: 'center'
      },
      tooltip: {},
      xAxis: {
        scale: true
      },
      yAxis: {},
      series: [
        {
          type: 'custom',
          renderItem: function (params, api) {
            var yValue = api.value(2);
            var start = api.coord([api.value(0), yValue]);
            var size = api.size([api.value(1) - api.value(0), yValue]);
            var style = api.style();
            return {
              type: 'rect',
              shape: {
                x: start[0],
                y: start[1],
                width: size[0],
                height: size[1]
              },
              style: style
            };
          },
          label: {
            show: true,
            position: 'top'
          },
          dimensions: ['from', 'to', 'profit'],
          encode: {
            x: [0, 1],
            y: 2,
            tooltip: [0, 1, 2],
            itemName: 3
          },
          data: data
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
