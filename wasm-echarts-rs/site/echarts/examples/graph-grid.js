/**
 * 官网示例：笛卡尔坐标系上的 Graph
 * https://echarts.apache.org/examples/zh/editor.html?c=graph-grid
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Graph on Cartesian
    category: graph
    titleCN: 笛卡尔坐标系上的 Graph
    difficulty: 2
    */
    const axisData = ['Mon', 'Tue', 'Wed', 'Very Loooong Thu', 'Fri', 'Sat', 'Sun'];
    const data = axisData.map(function (item, i) {
      return Math.round(Math.random() * 1000 * (i + 1));
    });
    const links = data.map(function (item, i) {
      return {
        source: i,
        target: i + 1
      };
    });
    links.pop();
    option = {
      title: {
        text: 'Graph on Cartesian'
      },
      tooltip: {},
      xAxis: {
        type: 'category',
        boundaryGap: false,
        data: axisData
      },
      yAxis: {
        type: 'value'
      },
      series: [
        {
          type: 'graph',
          layout: 'none',
          coordinateSystem: 'cartesian2d',
          symbolSize: 40,
          label: {
            show: true
          },
          edgeSymbol: ['circle', 'arrow'],
          edgeSymbolSize: [4, 10],
          data: data,
          links: links,
          lineStyle: {
            color: '#2f4554'
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
