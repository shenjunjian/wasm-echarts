/**
 * 官网示例：柱状图动画延迟
 * https://echarts.apache.org/examples/zh/editor.html?c=bar-animation-delay
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Animation Delay
    titleCN: 柱状图动画延迟
    category: bar
    difficulty: 5
    */
    var xAxisData = [];
    var data1 = [];
    var data2 = [];
    for (var i = 0; i < 100; i++) {
      xAxisData.push('A' + i);
      data1.push((Math.sin(i / 5) * (i / 5 - 10) + i / 6) * 5);
      data2.push((Math.cos(i / 5) * (i / 5 - 10) + i / 6) * 5);
    }
    option = {
      title: {
        text: 'Bar Animation Delay'
      },
      legend: {
        data: ['bar', 'bar2']
      },
      toolbox: {
        // y: 'bottom',
        feature: {
          magicType: {
            type: ['stack']
          },
          dataView: {},
          saveAsImage: {
            pixelRatio: 2
          }
        }
      },
      tooltip: {},
      xAxis: {
        data: xAxisData,
        splitLine: {
          show: false
        }
      },
      yAxis: {},
      series: [
        {
          name: 'bar',
          type: 'bar',
          data: data1,
          emphasis: {
            focus: 'series'
          },
          animationDelay: function (idx) {
            return idx * 10;
          }
        },
        {
          name: 'bar2',
          type: 'bar',
          data: data2,
          emphasis: {
            focus: 'series'
          },
          animationDelay: function (idx) {
            return idx * 10 + 100;
          }
        }
      ],
      animationEasing: 'elasticOut',
      animationDelayUpdate: function (idx) {
        return idx * 5;
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
