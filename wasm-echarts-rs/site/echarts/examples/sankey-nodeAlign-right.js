/**
 * 官网示例：桑基图右对齐布局
 * https://echarts.apache.org/examples/zh/editor.html?c=sankey-nodeAlign-right
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Node Align Right in Sankey
    category: sankey
    titleCN: 桑基图右对齐布局
    difficulty: 3
    */
    myChart.showLoading();
    $.get(ROOT_PATH + '/data/asset/data/energy.json', function (data) {
      myChart.hideLoading();
      myChart.setOption(
        (option = {
          title: {
            text: 'Node Align Right'
          },
          tooltip: {
            trigger: 'item',
            triggerOn: 'mousemove'
          },
          animation: false,
          series: [
            {
              type: 'sankey',
              emphasis: {
                focus: 'adjacency'
              },
              nodeAlign: 'right',
              data: data.nodes,
              links: data.links,
              lineStyle: {
                color: 'source',
                curveness: 0.5
              }
            }
          ]
        })
      );
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
