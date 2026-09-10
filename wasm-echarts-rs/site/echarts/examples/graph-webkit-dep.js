/**
 * 官网示例：WebKit 模块关系依赖图
 * https://echarts.apache.org/examples/zh/editor.html?c=graph-webkit-dep
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
  title: Graph Webkit Dep
  category: graph
  titleCN: WebKit 模块关系依赖图
  shotWidth: 900
  difficulty: 8
  */
  myChart.showLoading();
  myChart.showLoading();
  $.get(ROOT_PATH + '/data/asset/data/webkit-dep.json', function (webkitDep) {
    myChart.hideLoading();
    option = {
      legend: {
        data: ['HTMLElement', 'WebGL', 'SVG', 'CSS', 'Other']
      },
      series: [
        {
          type: 'graph',
          layout: 'force',
          animation: false,
          roam: true,
          roamTrigger: 'global',
          scaleLimit: {
            max: 8,
            min: 0.5
          },
          label: {
            position: 'right',
            formatter: '{b}'
          },
          draggable: true,
          data: webkitDep.nodes.map(function (node, idx) {
            node.id = idx;
            return node;
          }),
          categories: webkitDep.categories,
          force: {
            edgeLength: 5,
            repulsion: 20,
            gravity: 0.2
          },
          edges: webkitDep.links
        }
      ],
      thumbnail: {
        width: '15%',
        height: '15%',
        windowStyle: {
          color: 'rgba(140, 212, 250, 0.5)',
          borderColor: 'rgba(30, 64, 175, 0.7)',
          opacity: 1
        }
      }
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
