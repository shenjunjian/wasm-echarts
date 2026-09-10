/**
 * 官网示例：显示父层级标签
 * https://echarts.apache.org/examples/zh/editor.html?c=treemap-show-parent
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import initWasm, * as echarts from '@wasm-echarts';
import { ROOT_PATH, CDN_PATH, $, app, sizeCanvas, showPreviewError } from '../../src/echarts/official-env.js';
import { ensureDefaultFont } from '../../src/echarts/fonts.js';

async function main() {
  await initWasm();
  await ensureDefaultFont();

  const canvas = document.getElementById('canvas');
  if (!canvas) {
    throw new Error('缺少 #canvas');
  }
  sizeCanvas(canvas);
  const myChart = echarts.init(canvas);
  window.addEventListener('resize', () => {
    if (myChart.isDisposed()) return;
    sizeCanvas(canvas);
    myChart.resize();
  });

  let option;
  /*
  title: Show Parent Labels
  category: treemap
  titleCN: 显示父层级标签
  */
  myChart.showLoading();
  $.get(ROOT_PATH + '/data/asset/data/disk.tree.json', function (diskData) {
    myChart.hideLoading();
    function getLevelOption() {
      return [
        {
          itemStyle: {
            borderColor: '#777',
            borderWidth: 0,
            gapWidth: 1
          },
          upperLabel: {
            show: false
          }
        },
        {
          itemStyle: {
            borderColor: '#555',
            borderWidth: 5,
            gapWidth: 1
          },
          emphasis: {
            itemStyle: {
              borderColor: '#ddd'
            }
          }
        },
        {
          colorSaturation: [0.35, 0.5],
          itemStyle: {
            borderWidth: 5,
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
                echarts.format.encodeHTML(treePath.join('/')) +
                '</div>',
              'Disk Usage: ' + echarts.format.addCommas(value) + ' KB'
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
            upperLabel: {
              show: true,
              height: 30
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
  if (option) {
    myChart.setOption(option);
  }
}

main().catch((error) => {
  showPreviewError(error);
  console.error(error);
});
