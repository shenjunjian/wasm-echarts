/**
 * 官网示例：桑基图渐变色边
 * https://echarts.apache.org/examples/zh/editor.html?c=sankey-energy
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Gradient Edge
    category: sankey
    titleCN: 桑基图渐变色边
    difficulty: 3
    */
    myChart.showLoading();
    $.get(ROOT_PATH + '/data/asset/data/energy.json', function (data) {
      myChart.hideLoading();
      myChart.setOption(
        (option = {
          title: {
            text: 'Sankey Diagram'
          },
          tooltip: {
            trigger: 'item',
            triggerOn: 'mousemove'
          },
          series: [
            {
              type: 'sankey',
              data: data.nodes,
              links: data.links,
              emphasis: {
                focus: 'adjacency'
              },
              lineStyle: {
                color: 'gradient',
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
