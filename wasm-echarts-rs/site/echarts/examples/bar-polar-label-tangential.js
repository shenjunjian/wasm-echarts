/**
 * 官网示例：极坐标柱状图标签
 * https://echarts.apache.org/examples/zh/editor.html?c=bar-polar-label-tangential
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Tangential Polar Bar Label Position
    titleCN: 极坐标柱状图标签
    category: bar
    difficulty: 2
    */
    option = {
      title: [
        {
          text: 'Tangential Polar Bar Label Position (middle)'
        }
      ],
      polar: {
        radius: [30, '80%']
      },
      angleAxis: {
        max: 4,
        startAngle: 75
      },
      radiusAxis: {
        type: 'category',
        data: ['a', 'b', 'c', 'd']
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
