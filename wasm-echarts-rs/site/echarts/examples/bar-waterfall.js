/**
 * 官网示例：瀑布图（柱状图模拟）
 * https://echarts.apache.org/examples/zh/editor.html?c=bar-waterfall
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Waterfall Chart
    titleCN: 瀑布图（柱状图模拟）
    category: bar
    difficulty: 1
    */
    option = {
      title: {
        text: 'Waterfall Chart',
        subtext: 'Living Expenses in Shenzhen'
      },
      tooltip: {
        trigger: 'axis',
        axisPointer: {
          type: 'shadow'
        },
        formatter: function (params) {
          var tar = params[1];
          return tar.name + '<br/>' + tar.seriesName + ' : ' + tar.value;
        }
      },
      grid: {
        left: '3%',
        right: '4%',
        bottom: '3%',
        containLabel: true
      },
      xAxis: {
        type: 'category',
        splitLine: { show: false },
        data: ['Total', 'Rent', 'Utilities', 'Transportation', 'Meals', 'Other']
      },
      yAxis: {
        type: 'value'
      },
      series: [
        {
          name: 'Placeholder',
          type: 'bar',
          stack: 'Total',
          itemStyle: {
            borderColor: 'transparent',
            color: 'transparent'
          },
          emphasis: {
            itemStyle: {
              borderColor: 'transparent',
              color: 'transparent'
            }
          },
          data: [0, 1700, 1400, 1200, 300, 0]
        },
        {
          name: 'Life Cost',
          type: 'bar',
          stack: 'Total',
          label: {
            show: true,
            position: 'inside'
          },
          data: [2900, 1200, 300, 200, 900, 300]
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
