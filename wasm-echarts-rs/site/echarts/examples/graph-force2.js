/**
 * 官网示例：力引导布局
 * https://echarts.apache.org/examples/zh/editor.html?c=graph-force2
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
  title: Force Layout
  category: graph
  titleCN: 力引导布局
  difficulty: 1
  */
  function createNodes(count) {
    var nodes = [];
    for (var i = 0; i < count; i++) {
      nodes.push({
        id: i + ''
      });
    }
    return nodes;
  }
  function createEdges(count) {
    var edges = [];
    if (count === 2) {
      return [[0, 1]];
    }
    for (var i = 0; i < count; i++) {
      edges.push([i, (i + 1) % count]);
    }
    return edges;
  }
  var datas = [];
  for (var i = 0; i < 16; i++) {
    datas.push({
      nodes: createNodes(i + 2),
      edges: createEdges(i + 2)
    });
  }
  option = {
    series: datas.map(function (item, idx) {
      return {
        type: 'graph',
        layout: 'force',
        animation: false,
        data: item.nodes,
        left: (idx % 4) * 25 + '%',
        top: Math.floor(idx / 4) * 25 + '%',
        width: '25%',
        height: '25%',
        force: {
          // initLayout: 'circular'
          // gravity: 0
          repulsion: 60,
          edgeLength: 2
        },
        edges: item.edges.map(function (e) {
          return {
            source: e[0] + '',
            target: e[1] + ''
          };
        })
      };
    })
  };
  if (option) {
    myChart.setOption(option);
  }
}

main().catch((error) => {
  showPreviewError(error);
  console.error(error);
});
