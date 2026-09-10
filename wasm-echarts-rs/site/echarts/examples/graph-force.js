/**
 * 官网示例：力引导布局
 * https://echarts.apache.org/examples/zh/editor.html?c=graph-force
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Force Layout
    category: graph
    titleCN: 力引导布局
    difficulty: 3
    */
    myChart.showLoading();
    $.get(ROOT_PATH + '/data/asset/data/les-miserables.json', function (graph) {
      myChart.hideLoading();
      graph.nodes.forEach(function (node) {
        node.symbolSize = 5;
      });
      option = {
        title: {
          text: 'Les Miserables',
          subtext: 'Default layout',
          top: 'bottom',
          left: 'right'
        },
        tooltip: {},
        legend: [
          {
            // selectedMode: 'single',
            data: graph.categories.map(function (a) {
              return a.name;
            })
          }
        ],
        series: [
          {
            name: 'Les Miserables',
            type: 'graph',
            layout: 'force',
            data: graph.nodes,
            links: graph.links,
            categories: graph.categories,
            roam: true,
            label: {
              position: 'right'
            },
            force: {
              repulsion: 100
            }
          }
        ]
      };
      myChart.setOption(option);
    });
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
