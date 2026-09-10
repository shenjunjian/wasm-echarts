/**
 * 官网示例：极坐标柱状图标签
 * https://echarts.apache.org/examples/zh/editor.html?c=bar-polar-label-radial
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Radial Polar Bar Label Position
    titleCN: 极坐标柱状图标签
    category: bar
    difficulty: 2
    */
    option = {
      title: [
        {
          text: 'Radial Polar Bar Label Position (middle)'
        }
      ],
      polar: {
        radius: [30, '80%']
      },
      radiusAxis: {
        max: 4
      },
      angleAxis: {
        type: 'category',
        data: ['a', 'b', 'c', 'd'],
        startAngle: 75
      },
      tooltip: {},
      series: {
        type: 'bar',
        data: [2, 1.2, 2.4, 3.6],
        coordinateSystem: 'polar',
        label: {
          show: true,
          position: 'middle',
          formatter: '{b}: {c}'
        }
      },
      animation: false
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
