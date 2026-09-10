/**
 * 官网示例：堆叠柱状图的归一化和变化
 * https://echarts.apache.org/examples/zh/editor.html?c=bar-stack-normalization-and-variation
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
  title: Stacked Bar Normalization and Variation
  titleCN: 堆叠柱状图的归一化和变化
  category: bar
  difficulty: 3
  */
  // There should not be negative values in rawData
  const rawData = [
    [100, 302, 301, 334, 390, 330, 320],
    [320, 132, 101, 134, 90, 230, 210],
    [220, 182, 191, 234, 290, 330, 310],
    [150, 212, 201, 154, 190, 330, 410],
    [820, 832, 901, 934, 1290, 1330, 1320]
  ];
  const totalData = [];
  for (let i = 0; i < rawData[0].length; ++i) {
    let sum = 0;
    for (let j = 0; j < rawData.length; ++j) {
      sum += rawData[j][i];
    }
    totalData.push(sum);
  }
  const grid = {
    left: 100,
    right: 100,
    top: 50,
    bottom: 90
  };
  const gridWidth = myChart.getWidth() - grid.left - grid.right;
  const gridHeight = myChart.getHeight() - grid.top - grid.bottom;
  const categoryWidth = gridWidth / rawData[0].length;
  const barWidth = categoryWidth * 0.6;
  const barPadding = (categoryWidth - barWidth) / 2;
  const series = [
    'Direct',
    'Mail Ad',
    'Affiliate Ad',
    'Video Ad',
    'Search Engine'
  ].map((name, sid) => {
    return {
      name,
      type: 'bar',
      stack: 'total',
      barWidth: '60%',
      label: {
        show: true,
        formatter: (params) => Math.round(params.value * 1000) / 10 + '%'
      },
      data: rawData[sid].map((d, did) =>
        totalData[did] <= 0 ? 0 : d / totalData[did]
      )
    };
  });
  const color = ['#5070dd', '#b6d634', '#505372', '#ff994d', '#0ca8df'];
  const elements = [];
  for (let j = 1, jlen = rawData[0].length; j < jlen; ++j) {
    const leftX = grid.left + categoryWidth * j - barPadding;
    const rightX = leftX + barPadding * 2;
    let leftY = grid.top + gridHeight;
    let rightY = leftY;
    for (let i = 0, len = series.length; i < len; ++i) {
      const points = [];
      const leftBarHeight = (rawData[i][j - 1] / totalData[j - 1]) * gridHeight;
      points.push([leftX, leftY]);
      points.push([leftX, leftY - leftBarHeight]);
      const rightBarHeight = (rawData[i][j] / totalData[j]) * gridHeight;
      points.push([rightX, rightY - rightBarHeight]);
      points.push([rightX, rightY]);
      points.push([leftX, leftY]);
      leftY -= leftBarHeight;
      rightY -= rightBarHeight;
      elements.push({
        type: 'polygon',
        shape: {
          points
        },
        style: {
          fill: color[i],
          opacity: 0.25
        }
      });
    }
  }
  option = {
    legend: {
      selectedMode: false
    },
    grid,
    yAxis: {
      type: 'value'
    },
    xAxis: {
      type: 'category',
      data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun']
    },
    series,
    graphic: {
      elements
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
