/**
 * 官网示例：纵向日历图
 * https://echarts.apache.org/examples/zh/editor.html?c=calendar-vertical
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
  title: Calendar Heatmap Vertical
  category: 'calendar, heatmap'
  titleCN: 纵向日历图
  shotWidth: 900
  difficulty: 1
  */
  function getVirtualData(year) {
    const date = +echarts.time.parse(year + '-01-01');
    const end = +echarts.time.parse(+year + 1 + '-01-01');
    const dayTime = 3600 * 24 * 1000;
    const data = [];
    for (let time = date; time < end; time += dayTime) {
      data.push([
        echarts.time.format(time, '{yyyy}-{MM}-{dd}', false),
        Math.floor(Math.random() * 1000)
      ]);
    }
    return data;
  }
  option = {
    tooltip: {
      position: 'top',
      formatter: function (p) {
        const format = echarts.time.format(p.data[0], '{yyyy}-{MM}-{dd}', false);
        return format + ': ' + p.data[1];
      }
    },
    visualMap: {
      min: 0,
      max: 1000,
      calculable: true,
      orient: 'vertical',
      left: '670',
      top: 'center'
    },
    calendar: [
      {
        orient: 'vertical',
        range: '2015'
      },
      {
        left: 300,
        orient: 'vertical',
        range: '2016'
      },
      {
        left: 520,
        cellSize: [20, 'auto'],
        bottom: 10,
        orient: 'vertical',
        range: '2017',
        dayLabel: {
          margin: 5
        }
      }
    ],
    series: [
      {
        type: 'heatmap',
        coordinateSystem: 'calendar',
        calendarIndex: 0,
        data: getVirtualData('2015')
      },
      {
        type: 'heatmap',
        coordinateSystem: 'calendar',
        calendarIndex: 1,
        data: getVirtualData('2016')
      },
      {
        type: 'heatmap',
        coordinateSystem: 'calendar',
        calendarIndex: 2,
        data: getVirtualData('2017')
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
