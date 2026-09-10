/**
 * 官网示例：热力图与百度地图扩展
 * https://echarts.apache.org/examples/zh/editor.html?c=heatmap-bmap
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
  title: Heatmap on Baidu Map Extension
  category: heatmap
  tags: bmap
  noExplore: true
  titleCN: 热力图与百度地图扩展
  difficulty: 3
  */
  $.get(ROOT_PATH + '/data/asset/data/hangzhou-tracks.json', function (data) {
    var points = [].concat.apply(
      [],
      data.map(function (track) {
        return track.map(function (seg) {
          return seg.coord.concat([1]);
        });
      })
    );
    myChart.setOption(
      (option = {
        animation: false,
        bmap: {
          center: [120.13066322374, 30.240018034923],
          zoom: 14,
          roam: true
        },
        visualMap: {
          show: false,
          top: 'top',
          min: 0,
          max: 5,
          seriesIndex: 0,
          calculable: true,
          inRange: {
            color: ['blue', 'blue', 'green', 'yellow', 'red']
          }
        },
        series: [
          {
            type: 'heatmap',
            coordinateSystem: 'bmap',
            data: points,
            pointSize: 5,
            blurSize: 6
          }
        ]
      })
    );
    // 添加百度地图插件
    var bmap = myChart.getModel().getComponent('bmap').getBMap();
    bmap.addControl(new BMap.MapTypeControl());
  });
  if (option) {
    myChart.setOption(option);
  }
}

main().catch((error) => {
  showPreviewError(error);
  console.error(error);
});
