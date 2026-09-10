/**
 * 官网示例：柱状图框选
 * https://echarts.apache.org/examples/zh/editor.html?c=bar-brush
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
  title: Brush Select on Column Chart
  titleCN: 柱状图框选
  category: bar
  difficulty: 4
  */
  let xAxisData = [];
  let data1 = [];
  let data2 = [];
  let data3 = [];
  let data4 = [];
  for (let i = 0; i < 10; i++) {
    xAxisData.push('Class' + i);
    data1.push(+(Math.random() * 2).toFixed(2));
    data2.push(+(Math.random() * 5).toFixed(2));
    data3.push(+(Math.random() + 0.3).toFixed(2));
    data4.push(+Math.random().toFixed(2));
  }
  var emphasisStyle = {
    itemStyle: {
      shadowBlur: 10,
      shadowColor: 'rgba(0,0,0,0.3)'
    }
  };
  option = {
    legend: {
      data: ['bar', 'bar2', 'bar3', 'bar4'],
      left: '10%'
    },
    brush: {
      toolbox: ['rect', 'polygon', 'lineX', 'lineY', 'keep', 'clear'],
      xAxisIndex: 0
    },
    toolbox: {
      feature: {
        magicType: {
          type: ['stack']
        },
        dataView: {}
      }
    },
    tooltip: {},
    xAxis: {
      data: xAxisData,
      name: 'X Axis',
      axisLine: { onZero: true },
      splitLine: { show: false },
      splitArea: { show: false }
    },
    yAxis: {},
    grid: {
      bottom: 100
    },
    series: [
      {
        name: 'bar',
        type: 'bar',
        stack: 'one',
        emphasis: emphasisStyle,
        data: data1
      },
      {
        name: 'bar2',
        type: 'bar',
        stack: 'one',
        emphasis: emphasisStyle,
        data: data2
      },
      {
        name: 'bar3',
        type: 'bar',
        stack: 'two',
        emphasis: emphasisStyle,
        data: data3
      },
      {
        name: 'bar4',
        type: 'bar',
        stack: 'two',
        emphasis: emphasisStyle,
        data: data4
      }
    ]
  };
  myChart.on('brushSelected', function (params) {
    var brushed = [];
    var brushComponent = params.batch[0];
    for (var sIdx = 0; sIdx < brushComponent.selected.length; sIdx++) {
      var rawIndices = brushComponent.selected[sIdx].dataIndex;
      brushed.push('[Series ' + sIdx + '] ' + rawIndices.join(', '));
    }
    myChart.setOption({
      title: {
        backgroundColor: '#333',
        text: 'SELECTED DATA INDICES: \n' + brushed.join('\n'),
        bottom: 0,
        right: '10%',
        width: 100,
        textStyle: {
          fontSize: 12,
          color: '#fff'
        }
      }
    });
  });
  if (option) {
    myChart.setOption(option);
  }
}

main().catch((error) => {
  showPreviewError(error);
  console.error(error);
});
