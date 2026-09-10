/**
 * 官网示例：磁盘占用
 * https://echarts.apache.org/examples/zh/editor.html?c=treemap-disk
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Disk Usage
    category: treemap
    titleCN: 磁盘占用
    */
    myChart.showLoading();
    $.get(ROOT_PATH + '/data/asset/data/disk.tree.json', function (diskData) {
      myChart.hideLoading();
      const formatUtil = echarts.format;
      function getLevelOption() {
        return [
          {
            itemStyle: {
              borderWidth: 0,
              gapWidth: 5
            }
          },
          {
            itemStyle: {
              gapWidth: 1
            }
          },
          {
            colorSaturation: [0.35, 0.5],
            itemStyle: {
              gapWidth: 1,
              borderColorSaturation: 0.6
            }
          }
        ];
      }
      myChart.setOption(
        (option = {
          title: {
            text: 'Disk Usage',
            left: 'center'
          },
          tooltip: {
            formatter: function (info) {
              var value = info.value;
              var treePathInfo = info.treePathInfo;
              var treePath = [];
              for (var i = 1; i < treePathInfo.length; i++) {
                treePath.push(treePathInfo[i].name);
              }
              return [
                '<div class="tooltip-title">' +
                  formatUtil.encodeHTML(treePath.join('/')) +
                  '</div>',
                'Disk Usage: ' + formatUtil.addCommas(value) + ' KB'
              ].join('');
            }
          },
          series: [
            {
              name: 'Disk Usage',
              type: 'treemap',
              visibleMin: 300,
              label: {
                show: true,
                formatter: '{b}'
              },
              itemStyle: {
                borderColor: '#fff'
              },
              levels: getLevelOption(),
              data: diskData
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
