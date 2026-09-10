/**
 * 官网示例：从上到下树状图
 * https://echarts.apache.org/examples/zh/editor.html?c=tree-vertical
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: From Top to Bottom Tree
    category: tree
    titleCN: 从上到下树状图
    */
    myChart.showLoading();
    $.get(ROOT_PATH + '/data/asset/data/flare.json', function (data) {
      myChart.hideLoading();
      myChart.setOption(
        (option = {
          tooltip: {
            trigger: 'item',
            triggerOn: 'mousemove'
          },
          series: [
            {
              type: 'tree',
              data: [data],
              left: '2%',
              right: '2%',
              top: '8%',
              bottom: '20%',
              symbol: 'emptyCircle',
              orient: 'vertical',
              expandAndCollapse: true,
              label: {
                position: 'top',
                rotate: -90,
                verticalAlign: 'middle',
                align: 'right',
                fontSize: 9
              },
              leaves: {
                label: {
                  position: 'bottom',
                  rotate: -90,
                  verticalAlign: 'middle',
                  align: 'left'
                }
              },
              animationDurationUpdate: 750
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
