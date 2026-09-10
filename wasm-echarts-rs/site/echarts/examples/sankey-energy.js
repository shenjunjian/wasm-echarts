/**
 * 官网示例：桑基图渐变色边
 * https://echarts.apache.org/examples/zh/editor.html?c=sankey-energy
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
  if (option) {
    myChart.setOption(option);
  }
}

main().catch((error) => {
  showPreviewError(error);
  console.error(error);
});
