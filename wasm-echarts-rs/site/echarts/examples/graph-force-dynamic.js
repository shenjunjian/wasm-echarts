/**
 * 官网示例：动态增加图节点
 * https://echarts.apache.org/examples/zh/editor.html?c=graph-force-dynamic
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
  title: Graph Dynamic
  category: graph
  shotDelay: 5000
  titleCN: 动态增加图节点
  difficulty: 6
  */
  const data = [
    {
      fixed: true,
      x: myChart.getWidth() / 2,
      y: myChart.getHeight() / 2,
      symbolSize: 20,
      id: '-1'
    }
  ];
  const edges = [];
  option = {
    series: [
      {
        type: 'graph',
        layout: 'force',
        animation: false,
        data: data,
        force: {
          // initLayout: 'circular'
          // gravity: 0
          repulsion: 100,
          edgeLength: 5
        },
        edges: edges
      }
    ]
  };
  setInterval(function () {
    data.push({
      id: data.length + ''
    });
    var source = Math.round((data.length - 1) * Math.random());
    var target = Math.round((data.length - 1) * Math.random());
    if (source !== target) {
      edges.push({
        source: source,
        target: target
      });
    }
    myChart.setOption({
      series: [
        {
          roam: true,
          data: data,
          edges: edges
        }
      ]
    });
    // console.log('nodes: ' + data.length);
    // console.log('links: ' + data.length);
  }, 200);
  if (option) {
    myChart.setOption(option);
  }
}

main().catch((error) => {
  showPreviewError(error);
  console.error(error);
});
