/**
 * 官网示例：从右到左树状图
 * https://echarts.apache.org/examples/zh/editor.html?c=tree-orient-right-left
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
  title: From Right to Left Tree
  category: tree
  titleCN: 从右到左树状图
  */
  myChart.showLoading();
  $.get(ROOT_PATH + '/data/asset/data/flare.json', function (data) {
    myChart.hideLoading();
    data.children.forEach(function (datum, index) {
      index % 2 === 0 && (datum.collapsed = true);
    });
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
            top: '1%',
            left: '15%',
            bottom: '1%',
            right: '7%',
            symbolSize: 7,
            orient: 'RL',
            label: {
              position: 'right',
              verticalAlign: 'middle',
              align: 'left'
            },
            leaves: {
              label: {
                position: 'left',
                verticalAlign: 'middle',
                align: 'right'
              }
            },
            emphasis: {
              focus: 'descendant'
            },
            expandAndCollapse: true,
            animationDuration: 550,
            animationDurationUpdate: 750
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
