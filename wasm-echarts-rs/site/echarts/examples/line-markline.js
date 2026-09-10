/**
 * 官网示例：折线图的标记线
 * https://echarts.apache.org/examples/zh/editor.html?c=line-markline
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
  title: Line with Marklines
  titleCN: 折线图的标记线
  category: line
  difficulty: 6
  */
  const markLine = [];
  const positions = [
    'start',
    'middle',
    'end',
    'insideStart',
    'insideStartTop',
    'insideStartBottom',
    'insideMiddle',
    'insideMiddleTop',
    'insideMiddleBottom',
    'insideEnd',
    'insideEndTop',
    'insideEndBottom'
  ];
  for (var i = 0; i < positions.length; ++i) {
    markLine.push({
      name: positions[i],
      yAxis: 1.8 - 0.2 * Math.floor(i / 3),
      label: {
        formatter: '{b}',
        position: positions[i]
      }
    });
    if (positions[i] !== 'middle') {
      const name =
        positions[i] === 'insideMiddle' ? 'insideMiddle / middle' : positions[i];
      markLine.push([
        {
          name: 'start: ' + positions[i],
          coord: [0, 0.3],
          label: {
            formatter: name,
            position: positions[i]
          }
        },
        {
          name: 'end: ' + positions[i],
          coord: [3, 1]
        }
      ]);
    }
  }
  option = {
    animation: false,
    textStyle: {
      fontSize: 14
    },
    xAxis: {
      data: ['A', 'B', 'C', 'D', 'E'],
      boundaryGap: true,
      splitArea: {
        show: true
      }
    },
    yAxis: {
      max: 2
    },
    series: [
      {
        name: 'line',
        type: 'line',
        stack: 'all',
        symbolSize: 6,
        data: [0.3, 1.4, 1.2, 1, 0.6],
        markLine: {
          data: markLine,
          label: {
            distance: [20, 8]
          }
        }
      }
    ],
    grid: {
      top: 30,
      left: 60,
      right: 60,
      bottom: 40
    }
  };
  if (option) {
    myChart.setOption(option);
  }
}

main().catch((error) => {
  showPreviewError(error);
  console.error(error);
});
