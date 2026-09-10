/**
 * 官网示例：桑基图右对齐布局
 * https://echarts.apache.org/examples/zh/editor.html?c=sankey-nodeAlign-right
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
  if (option) {
    myChart.setOption(option);
  }
}

main().catch((error) => {
  showPreviewError(error);
  console.error(error);
});
