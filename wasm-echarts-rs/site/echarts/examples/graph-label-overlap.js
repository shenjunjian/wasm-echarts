/**
 * 官网示例：关系图自动隐藏重叠标签
 * https://echarts.apache.org/examples/zh/editor.html?c=graph-label-overlap
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Hide Overlapped Label
    category: graph
    titleCN: 关系图自动隐藏重叠标签
    difficulty: 3
    */
    myChart.showLoading();
    $.getJSON(ROOT_PATH + '/data/asset/data/les-miserables.json', function (graph) {
      myChart.hideLoading();
      option = {
        tooltip: {},
        legend: [
          {
            data: graph.categories.map(function (a) {
              return a.name;
            })
          }
        ],
        series: [
          {
            name: 'Les Miserables',
            type: 'graph',
            layout: 'none',
            data: graph.nodes,
            links: graph.links,
            categories: graph.categories,
            roam: true,
            label: {
              show: true,
              position: 'right',
              formatter: '{b}'
            },
            labelLayout: {
              hideOverlap: true
            },
            scaleLimit: {
              min: 0.4,
              max: 2
            },
            lineStyle: {
              color: 'source',
              curveness: 0.3
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
