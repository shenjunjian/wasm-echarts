/**
 * 官网示例：悲惨世界人物关系图(环形布局)
 * https://echarts.apache.org/examples/zh/editor.html?c=graph-circular-layout
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
  title: Les Miserables
  category: graph
  titleCN: 悲惨世界人物关系图(环形布局)
  shotWidth: 900
  difficulty: 5
  */
  myChart.showLoading();
  $.getJSON(ROOT_PATH + '/data/asset/data/les-miserables.json', function (graph) {
    myChart.hideLoading();
    graph.nodes.forEach(function (node) {
      node.label = {
        show: node.symbolSize > 30
      };
    });
    option = {
      title: {
        text: 'Les Miserables',
        subtext: 'Circular layout',
        top: 'bottom',
        left: 'right'
      },
      tooltip: {},
      legend: [
        {
          data: graph.categories.map(function (a) {
            return a.name;
          })
        }
      ],
      animationDurationUpdate: 1500,
      animationEasingUpdate: 'quinticInOut',
      series: [
        {
          name: 'Les Miserables',
          type: 'graph',
          layout: 'circular',
          circular: {
            rotateLabel: true
          },
          data: graph.nodes,
          links: graph.links,
          categories: graph.categories,
          roam: true,
          label: {
            position: 'right',
            formatter: '{b}'
          },
          lineStyle: {
            color: 'source',
            curveness: 0.3
          }
        }
      ]
    };
    myChart.setOption(option);
  });
  if (option) {
    myChart.setOption(option);
  }
}

main().catch((error) => {
  showPreviewError(error);
  console.error(error);
});
