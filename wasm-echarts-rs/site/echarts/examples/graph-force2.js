/**
 * 官网示例：力引导布局
 * https://echarts.apache.org/examples/zh/editor.html?c=graph-force2
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
    difficulty: 1
    */
    function createNodes(count) {
      var nodes = [];
      for (var i = 0; i < count; i++) {
        nodes.push({
          id: i + ''
        });
      }
      return nodes;
    }
    function createEdges(count) {
      var edges = [];
      if (count === 2) {
        return [[0, 1]];
      }
      for (var i = 0; i < count; i++) {
        edges.push([i, (i + 1) % count]);
      }
      return edges;
    }
    var datas = [];
    for (var i = 0; i < 16; i++) {
      datas.push({
        nodes: createNodes(i + 2),
        edges: createEdges(i + 2)
      });
    }
    option = {
      series: datas.map(function (item, idx) {
        return {
          type: 'graph',
          layout: 'force',
          animation: false,
          data: item.nodes,
          left: (idx % 4) * 25 + '%',
          top: Math.floor(idx / 4) * 25 + '%',
          width: '25%',
          height: '25%',
          force: {
            // initLayout: 'circular'
            // gravity: 0
            repulsion: 60,
            edgeLength: 2
          },
          edges: item.edges.map(function (e) {
            return {
              source: e[0] + '',
              target: e[1] + ''
            };
          })
        };
      })
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
