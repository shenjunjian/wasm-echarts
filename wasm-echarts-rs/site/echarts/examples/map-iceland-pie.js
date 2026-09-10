/**
 * 官网示例：在地图上显示饼图
 * https://echarts.apache.org/examples/zh/editor.html?c=map-iceland-pie
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
  title: Pie Charts on GEO Map
  category: map, pie
  titleCN: 在地图上显示饼图
  since: 5.4.0
  difficulty: 5
  */
  myChart.showLoading();
  $.get(ROOT_PATH + '/data/asset/geo/iceland.geo.json', function (geoJSON) {
    echarts.registerMap('iceland', geoJSON);
    function randomPieSeries(center, radius) {
      const data = ['A', 'B', 'C', 'D'].map((t) => {
        return {
          value: Math.round(Math.random() * 100),
          name: 'Category ' + t
        };
      });
      return {
        type: 'pie',
        coordinateSystem: 'geo',
        tooltip: {
          formatter: '{b}: {c} ({d}%)'
        },
        label: {
          show: false
        },
        labelLine: {
          show: false
        },
        animationDuration: 0,
        radius,
        center,
        data
      };
    }
    option = {
      geo: {
        map: 'iceland',
        roam: true,
        aspectScale: Math.cos((65 * Math.PI) / 180),
        // nameProperty: 'name_en', // If using en name.
        itemStyle: {
          areaColor: '#e7e8ea'
        },
        emphasis: {
          label: { show: false }
        }
      },
      tooltip: {},
      legend: {},
      series: [
        randomPieSeries([-19.007740346534653, 64.1780281585128], 45),
        randomPieSeries([-17.204666089108912, 65.44804833928391], 25),
        randomPieSeries([-15.264995297029705, 64.8592208009264], 30),
        randomPieSeries(
          // it's also supported to use geo region name as center since v5.4.1
          +echarts.version.split('.').slice(0, 3).join('') > 540
            ? 'Vestfirðir'
            : // or you can only use the LngLat array
              [-13, 66],
          30
        )
      ]
    };
    myChart.hideLoading();
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
