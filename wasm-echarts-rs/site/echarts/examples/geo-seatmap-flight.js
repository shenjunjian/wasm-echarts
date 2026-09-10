/**
 * 官网示例：航班选座（SVG）
 * https://echarts.apache.org/examples/zh/editor.html?c=geo-seatmap-flight
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
  title: Flight Seatmap with SVG
  category: map
  titleCN: 航班选座（SVG）
  */
  $.get(ROOT_PATH + '/data/asset/geo/flight-seats.svg', function (svg) {
    echarts.registerMap('flight-seats', { svg: svg });
    const takenSeatNames = ['26E', '26D', '26C', '25D', '23C', '21A', '20F'];
    option = {
      tooltip: {},
      geo: {
        map: 'flight-seats',
        roam: true,
        selectedMode: 'multiple',
        layoutCenter: ['50%', '50%'],
        layoutSize: '95%',
        tooltip: {
          show: true
        },
        itemStyle: {
          color: '#fff'
        },
        emphasis: {
          itemStyle: {
            color: undefined,
            borderColor: 'green',
            borderWidth: 2
          },
          label: {
            show: false
          }
        },
        select: {
          itemStyle: {
            color: 'green'
          },
          label: {
            show: false,
            textBorderColor: '#fff',
            textBorderWidth: 2
          }
        },
        regions: makeTakenRegions(takenSeatNames)
      }
    };
    function makeTakenRegions(takenSeatNames) {
      var regions = [];
      for (var i = 0; i < takenSeatNames.length; i++) {
        regions.push({
          name: takenSeatNames[i],
          silent: true,
          itemStyle: {
            color: '#bf0e08'
          },
          emphasis: {
            itemStyle: {
              borderColor: '#aaa',
              borderWidth: 1
            }
          },
          select: {
            itemStyle: {
              color: '#bf0e08'
            }
          }
        });
      }
      return regions;
    }
    myChart.setOption(option);
    // Get selected seats.
    myChart.on('geoselectchanged', function (params) {
      const selectedNames = params.allSelected[0].name.slice();
      // Remove taken seats.
      for (var i = selectedNames.length - 1; i >= 0; i--) {
        if (takenSeatNames.indexOf(selectedNames[i]) >= 0) {
          selectedNames.splice(i, 1);
        }
      }
      console.log('selected', selectedNames);
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
