/**
 * 官网示例：动态增加图节点
 * https://echarts.apache.org/examples/zh/editor.html?c=graph-force-dynamic
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Graph Dynamic
    category: graph
    shotDelay: 5000
    titleCN: 动态增加图节点
    difficulty: 6
    */
    const data = [
      {
        fixed: true,
        x: myChart.getWidth() / 2,
        y: myChart.getHeight() / 2,
        symbolSize: 20,
        id: '-1'
      }
    ];
    const edges = [];
    option = {
      series: [
        {
          type: 'graph',
          layout: 'force',
          animation: false,
          data: data,
          force: {
            // initLayout: 'circular'
            // gravity: 0
            repulsion: 100,
            edgeLength: 5
          },
          edges: edges
        }
      ]
    };
    setInterval(function () {
      data.push({
        id: data.length + ''
      });
      var source = Math.round((data.length - 1) * Math.random());
      var target = Math.round((data.length - 1) * Math.random());
      if (source !== target) {
        edges.push({
          source: source,
          target: target
        });
      }
      myChart.setOption({
        series: [
          {
            roam: true,
            data: data,
            edges: edges
          }
        ]
      });
      // console.log('nodes: ' + data.length);
      // console.log('links: ' + data.length);
    }, 200);
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
