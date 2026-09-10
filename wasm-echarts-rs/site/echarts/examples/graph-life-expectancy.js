/**
 * 官网示例：预期寿命
 * https://echarts.apache.org/examples/zh/editor.html?c=graph-life-expectancy
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
  title: Graph Life Expectancy
  category: graph
  titleCN: 预期寿命
  difficulty: 7
  */
  $.get(ROOT_PATH + '/data/asset/data/life-expectancy.json', function (rawData) {
    const series = [];
    rawData.counties.forEach(function (country) {
      const data = rawData.series.map(function (yearData) {
        const item = yearData.filter(function (item) {
          return item[3] === country;
        })[0];
        return {
          label: {
            show: +item[4] % 20 === 0 && +item[4] > 1940,
            position: 'top'
          },
          emphasis: {
            label: {
              show: true
            }
          },
          name: item[4],
          value: item
        };
      });
      var links = data.map(function (item, idx) {
        return {
          source: idx,
          target: idx + 1
        };
      });
      links.pop();
      series.push({
        name: country,
        type: 'graph',
        coordinateSystem: 'cartesian2d',
        data: data,
        links: links,
        edgeSymbol: ['none', 'arrow'],
        edgeSymbolSize: 5,
        legendHoverLink: false,
        lineStyle: {
          color: '#333'
        },
        itemStyle: {
          borderWidth: 1,
          borderColor: '#333'
        },
        label: {
          color: '#333',
          position: 'right'
        },
        symbolSize: 10,
        animationDelay: function (idx) {
          return idx * 100;
        }
      });
    });
    option = {
      visualMap: {
        show: false,
        min: 0,
        max: 100,
        dimension: 1
      },
      legend: {
        data: rawData.counties,
        selectedMode: 'single',
        right: 100
      },
      grid: {
        left: 0,
        bottom: 0,
        containLabel: true,
        top: 80
      },
      xAxis: {
        type: 'value'
      },
      yAxis: {
        type: 'value',
        scale: true
      },
      toolbox: {
        feature: {
          dataZoom: {}
        }
      },
      dataZoom: {
        type: 'inside'
      },
      series: series
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
