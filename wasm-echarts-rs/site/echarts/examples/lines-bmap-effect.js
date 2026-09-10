/**
 * 官网示例：北京公交路线 - 线特效
 * https://echarts.apache.org/examples/zh/editor.html?c=lines-bmap-effect
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
  title: Bus Lines of Beijing - Line Effect
  category: 'map, lines'
  noExplore: true
  tags: bmap
  titleCN: 北京公交路线 - 线特效
  shotDelay: 2000
  */
  $.get(ROOT_PATH + '/data/asset/data/lines-bus.json', function (data) {
    let hStep = 300 / (data.length - 1);
    let busLines = [].concat.apply(
      [],
      data.map(function (busLine, idx) {
        let prevPt = [];
        let points = [];
        for (let i = 0; i < busLine.length; i += 2) {
          let pt = [busLine[i], busLine[i + 1]];
          if (i > 0) {
            pt = [prevPt[0] + pt[0], prevPt[1] + pt[1]];
          }
          prevPt = pt;
          points.push([pt[0] / 1e4, pt[1] / 1e4]);
        }
        return {
          coords: points,
          lineStyle: {
            normal: {
              color: echarts.color.modifyHSL('#5A94DF', Math.round(hStep * idx))
            }
          }
        };
      })
    );
    myChart.setOption(
      (option = {
        backgroundColor: 'transparent',
        bmap: {
          center: [116.46, 39.92],
          zoom: 10,
          roam: true,
          mapStyle: {
            styleJson: [
              {
                featureType: 'water',
                elementType: 'all',
                stylers: {
                  color: '#031628'
                }
              },
              {
                featureType: 'land',
                elementType: 'geometry',
                stylers: {
                  color: '#000102'
                }
              },
              {
                featureType: 'highway',
                elementType: 'all',
                stylers: {
                  visibility: 'off'
                }
              },
              {
                featureType: 'arterial',
                elementType: 'geometry.fill',
                stylers: {
                  color: '#000000'
                }
              },
              {
                featureType: 'arterial',
                elementType: 'geometry.stroke',
                stylers: {
                  color: '#0b3d51'
                }
              },
              {
                featureType: 'local',
                elementType: 'geometry',
                stylers: {
                  color: '#000000'
                }
              },
              {
                featureType: 'railway',
                elementType: 'geometry.fill',
                stylers: {
                  color: '#000000'
                }
              },
              {
                featureType: 'railway',
                elementType: 'geometry.stroke',
                stylers: {
                  color: '#08304b'
                }
              },
              {
                featureType: 'subway',
                elementType: 'geometry',
                stylers: {
                  lightness: -70
                }
              },
              {
                featureType: 'building',
                elementType: 'geometry.fill',
                stylers: {
                  color: '#000000'
                }
              },
              {
                featureType: 'all',
                elementType: 'labels.text.fill',
                stylers: {
                  color: '#857f7f'
                }
              },
              {
                featureType: 'all',
                elementType: 'labels.text.stroke',
                stylers: {
                  color: '#000000'
                }
              },
              {
                featureType: 'building',
                elementType: 'geometry',
                stylers: {
                  color: '#022338'
                }
              },
              {
                featureType: 'green',
                elementType: 'geometry',
                stylers: {
                  color: '#062032'
                }
              },
              {
                featureType: 'boundary',
                elementType: 'all',
                stylers: {
                  color: '#465b6c'
                }
              },
              {
                featureType: 'manmade',
                elementType: 'all',
                stylers: {
                  color: '#022338'
                }
              },
              {
                featureType: 'label',
                elementType: 'all',
                stylers: {
                  visibility: 'off'
                }
              }
            ]
          }
        },
        series: [
          {
            type: 'lines',
            coordinateSystem: 'bmap',
            polyline: true,
            data: busLines,
            silent: true,
            lineStyle: {
              // color: '#c23531',
              // color: 'rgb(200, 35, 45)',
              opacity: 0.2,
              width: 1
            },
            progressiveThreshold: 500,
            progressive: 200
          },
          {
            type: 'lines',
            coordinateSystem: 'bmap',
            polyline: true,
            data: busLines,
            lineStyle: {
              width: 0
            },
            effect: {
              constantSpeed: 20,
              show: true,
              trailLength: 0.1,
              symbolSize: 1.5
            },
            zlevel: 1
          }
        ]
      })
    );
  });
  if (option) {
    myChart.setOption(option);
  }
}

main().catch((error) => {
  showPreviewError(error);
  console.error(error);
});
