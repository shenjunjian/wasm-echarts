/**
 * 官网示例：极坐标系下的堆叠柱状图
 * https://echarts.apache.org/examples/zh/editor.html?c=bar-polar-stack
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Stacked Bar Chart on Polar
    titleCN: 极坐标系下的堆叠柱状图
    category: bar
    difficulty: 7
    */
    option = {
      angleAxis: {},
      radiusAxis: {
        type: 'category',
        data: ['Mon', 'Tue', 'Wed', 'Thu'],
        z: 10
      },
      polar: {},
      series: [
        {
          type: 'bar',
          data: [1, 2, 3, 4],
          coordinateSystem: 'polar',
          name: 'A',
          stack: 'a',
          emphasis: {
            focus: 'series'
          }
        },
        {
          type: 'bar',
          data: [2, 4, 6, 8],
          coordinateSystem: 'polar',
          name: 'B',
          stack: 'a',
          emphasis: {
            focus: 'series'
          }
        },
        {
          type: 'bar',
          data: [1, 2, 3, 4],
          coordinateSystem: 'polar',
          name: 'C',
          stack: 'a',
          emphasis: {
            focus: 'series'
          }
        }
      ],
      legend: {
        show: true,
        data: ['A', 'B', 'C']
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
