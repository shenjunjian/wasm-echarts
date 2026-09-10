/**
 * 官网示例：大数据量柱图
 * https://echarts.apache.org/examples/zh/editor.html?c=bar-large
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
  const myChart = echarts.init(canvas, null, { useWorker: true });
  window.addEventListener('resize', () => {
    if (myChart.isDisposed()) return;
    sizeCanvas(canvas);
    myChart.resize();
  });

  let option;
  /*
  title: Large Scale Bar Chart
  category: bar
  titleCN: 大数据量柱图
  difficulty: 5
  */
  const dataCount = 5e5;
  const data = generateData(dataCount);
  option = {
    title: {
      text: echarts.format.addCommas(dataCount) + ' Data',
      left: 10
    },
    toolbox: {
      feature: {
        dataZoom: {
          yAxisIndex: false
        },
        saveAsImage: {
          pixelRatio: 2
        }
      }
    },
    tooltip: {
      trigger: 'axis',
      axisPointer: {
        type: 'shadow'
      }
    },
    grid: {
      bottom: 90
    },
    dataZoom: [
      {
        type: 'inside'
      },
      {
        type: 'slider'
      }
    ],
    xAxis: {
      data: data.categoryData,
      silent: false,
      splitLine: {
        show: false
      },
      splitArea: {
        show: false
      }
    },
    yAxis: {
      splitArea: {
        show: false
      }
    },
    series: [
      {
        type: 'bar',
        data: data.valueData,
        // Set `large` for large data amount
        large: true
      }
    ]
  };
  function generateData(count) {
    let baseValue = Math.random() * 1000;
    let time = +new Date(2011, 0, 1);
    let smallBaseValue;
    function next(idx) {
      smallBaseValue =
        idx % 30 === 0
          ? Math.random() * 700
          : smallBaseValue + Math.random() * 500 - 250;
      baseValue += Math.random() * 20 - 10;
      return Math.max(0, Math.round(baseValue + smallBaseValue) + 3000);
    }
    const categoryData = [];
    const valueData = [];
    for (let i = 0; i < count; i++) {
      categoryData.push(
        echarts.format.formatTime('yyyy-MM-dd\nhh:mm:ss', time, false)
      );
      valueData.push(next(i).toFixed(2));
      time += 1000;
    }
    return {
      categoryData: categoryData,
      valueData: valueData
    };
  }
  if (option) {
    await myChart.setOption(option);
  }
}

main().catch((error) => {
  showPreviewError(error);
  console.error(error);
});
