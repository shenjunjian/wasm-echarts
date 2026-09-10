/**
 * 官网折线示例：折线图的标记线
 * https://echarts.apache.org/examples/zh/editor.html?c=line-markline
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Line with Marklines
    titleCN: 折线图的标记线
    category: line
    difficulty: 6
    */
    const markLine = [];
    const positions = [
      'start',
      'middle',
      'end',
      'insideStart',
      'insideStartTop',
      'insideStartBottom',
      'insideMiddle',
      'insideMiddleTop',
      'insideMiddleBottom',
      'insideEnd',
      'insideEndTop',
      'insideEndBottom'
    ];
    for (var i = 0; i < positions.length; ++i) {
      markLine.push({
        name: positions[i],
        yAxis: 1.8 - 0.2 * Math.floor(i / 3),
        label: {
          formatter: '{b}',
          position: positions[i]
        }
      });
      if (positions[i] !== 'middle') {
        const name =
          positions[i] === 'insideMiddle' ? 'insideMiddle / middle' : positions[i];
        markLine.push([
          {
            name: 'start: ' + positions[i],
            coord: [0, 0.3],
            label: {
              formatter: name,
              position: positions[i]
            }
          },
          {
            name: 'end: ' + positions[i],
            coord: [3, 1]
          }
        ]);
      }
    }
    option = {
      animation: false,
      textStyle: {
        fontSize: 14
      },
      xAxis: {
        data: ['A', 'B', 'C', 'D', 'E'],
        boundaryGap: true,
        splitArea: {
          show: true
        }
      },
      yAxis: {
        max: 2
      },
      series: [
        {
          name: 'line',
          type: 'line',
          stack: 'all',
          symbolSize: 6,
          data: [0.3, 1.4, 1.2, 1, 0.6],
          markLine: {
            data: markLine,
            label: {
              distance: [20, 8]
            }
          }
        }
      ],
      grid: {
        top: 30,
        left: 60,
        right: 60,
        bottom: 40
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
