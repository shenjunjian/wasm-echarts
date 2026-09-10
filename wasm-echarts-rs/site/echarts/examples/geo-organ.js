/**
 * 官网示例：内脏数据（SVG）
 * https://echarts.apache.org/examples/zh/editor.html?c=geo-organ
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
  title: Organ Data with SVG
  category: map
  titleCN: 内脏数据（SVG）
  */
  $.get(
    ROOT_PATH + '/data/asset/geo/Veins_Medical_Diagram_clip_art.svg',
    function (svg) {
      echarts.registerMap('organ_diagram', { svg: svg });
      option = {
        tooltip: {},
        geo: {
          left: 10,
          right: '50%',
          map: 'organ_diagram',
          selectedMode: 'multiple',
          emphasis: {
            focus: 'self',
            itemStyle: {
              color: null
            },
            label: {
              position: 'bottom',
              distance: 0,
              textBorderColor: '#fff',
              textBorderWidth: 2
            }
          },
          blur: {},
          select: {
            itemStyle: {
              color: '#b50205'
            },
            label: {
              show: false,
              textBorderColor: '#fff',
              textBorderWidth: 2
            }
          }
        },
        grid: {
          left: '60%',
          top: '20%',
          bottom: '20%'
        },
        xAxis: {},
        yAxis: {
          data: [
            'heart',
            'large-intestine',
            'small-intestine',
            'spleen',
            'kidney',
            'lung',
            'liver'
          ]
        },
        series: [
          {
            type: 'bar',
            emphasis: {
              focus: 'self'
            },
            data: [121, 321, 141, 52, 198, 289, 139]
          }
        ]
      };
      myChart.setOption(option);
      myChart.on('mouseover', { seriesIndex: 0 }, function (event) {
        myChart.dispatchAction({
          type: 'highlight',
          geoIndex: 0,
          name: event.name
        });
      });
      myChart.on('mouseout', { seriesIndex: 0 }, function (event) {
        myChart.dispatchAction({
          type: 'downplay',
          geoIndex: 0,
          name: event.name
        });
      });
    }
  );
  if (option) {
    myChart.setOption(option);
  }
}

main().catch((error) => {
  showPreviewError(error);
  console.error(error);
});
