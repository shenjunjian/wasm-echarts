/**
 * 官网示例：NPM 依赖关系图
 * https://echarts.apache.org/examples/zh/editor.html?c=graph-npm
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: NPM Dependencies
    category: graph
    titleCN: NPM 依赖关系图
    difficulty: 9
    */
    myChart.showLoading();
    $.getJSON(
      ROOT_PATH + '/data/asset/data/npmdepgraph.min10.json',
      function (json) {
        myChart.hideLoading();
        myChart.setOption(
          (option = {
            title: {
              text: 'NPM Dependencies'
            },
            animationDurationUpdate: 1500,
            animationEasingUpdate: 'quinticInOut',
            series: [
              {
                type: 'graph',
                layout: 'none',
                // progressiveThreshold: 700,
                data: json.nodes.map(function (node) {
                  return {
                    x: node.x,
                    y: node.y,
                    id: node.id,
                    name: node.label,
                    symbolSize: node.size,
                    itemStyle: {
                      color: node.color
                    }
                  };
                }),
                edges: json.edges.map(function (edge) {
                  return {
                    source: edge.sourceID,
                    target: edge.targetID
                  };
                }),
                emphasis: {
                  focus: 'adjacency',
                  label: {
                    position: 'right',
                    show: true
                  }
                },
                roam: true,
                roamTrigger: 'global',
                lineStyle: {
                  width: 0.5,
                  curveness: 0.3,
                  opacity: 0.7
                }
              }
            ],
            thumbnail: {
              width: '20%',
              height: '20%',
              windowStyle: {
                color: 'rgba(140, 212, 250, 0.5)',
                borderColor: 'rgba(30, 64, 175, 0.7)',
                opacity: 1
              }
            }
          }),
          true
        );
      }
    );
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
