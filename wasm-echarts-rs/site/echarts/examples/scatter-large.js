/**
 * 官网示例：大规模散点图
 * https://echarts.apache.org/examples/zh/editor.html?c=scatter-large
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
  title: Large Scatter
  category: scatter
  titleCN: 大规模散点图
  difficulty: 5
  */
  function genData(len, offset) {
    let arr = new Float32Array(len * 2);
    let off = 0;
    for (let i = 0; i < len; i++) {
      let x = +Math.random() * 10;
      let y =
        +Math.sin(x) -
        x * (len % 2 ? 0.1 : -0.1) * Math.random() +
        (offset || 0) / 10;
      arr[off++] = x;
      arr[off++] = y;
    }
    return arr;
  }
  const data1 = genData(5e5);
  const data2 = genData(5e5, 10);
  option = {
    title: {
      text:
        echarts.format.addCommas(data1.length / 2 + data2.length / 2) + ' Points'
    },
    tooltip: {},
    toolbox: {
      left: 'center',
      feature: {
        dataZoom: {}
      }
    },
    legend: {
      orient: 'vertical',
      right: 10
    },
    xAxis: [{}],
    yAxis: [{}],
    dataZoom: [
      {
        type: 'inside'
      },
      {
        type: 'slider'
      }
    ],
    animation: false,
    series: [
      {
        name: 'A',
        type: 'scatter',
        data: data1,
        dimensions: ['x', 'y'],
        symbolSize: 3,
        itemStyle: {
          opacity: 0.4
        },
        large: true
      },
      {
        name: 'B',
        type: 'scatter',
        data: data2,
        dimensions: ['x', 'y'],
        symbolSize: 3,
        itemStyle: {
          opacity: 0.4
        },
        large: true
      }
    ]
  };
  if (option) {
    myChart.setOption(option);
  }
}

main().catch((error) => {
  showPreviewError(error);
  console.error(error);
});
