/**
 * 官网示例：柱状图下钻动画
 * https://echarts.apache.org/examples/zh/editor.html?c=bar-drilldown
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
  title: Bar Chart Drilldown Animation
  category: bar
  titleCN: 柱状图下钻动画
  difficulty: 5
  */
  option = {
    xAxis: {
      data: ['Animals', 'Fruits', 'Cars']
    },
    yAxis: {},
    dataGroupId: '',
    animationDurationUpdate: 500,
    series: {
      type: 'bar',
      id: 'sales',
      data: [
        {
          value: 5,
          groupId: 'animals'
        },
        {
          value: 2,
          groupId: 'fruits'
        },
        {
          value: 4,
          groupId: 'cars'
        }
      ],
      universalTransition: {
        enabled: true,
        divideShape: 'clone'
      }
    }
  };
  const drilldownData = [
    {
      dataGroupId: 'animals',
      data: [
        ['Cats', 4],
        ['Dogs', 2],
        ['Cows', 1],
        ['Sheep', 2],
        ['Pigs', 1]
      ]
    },
    {
      dataGroupId: 'fruits',
      data: [
        ['Apples', 4],
        ['Oranges', 2]
      ]
    },
    {
      dataGroupId: 'cars',
      data: [
        ['Toyota', 4],
        ['Opel', 2],
        ['Volkswagen', 2]
      ]
    }
  ];
  myChart.on('click', function (event) {
    if (event.data) {
      var subData = drilldownData.find(function (data) {
        return data.dataGroupId === event.data.groupId;
      });
      if (!subData) {
        return;
      }
      myChart.setOption({
        xAxis: {
          data: subData.data.map(function (item) {
            return item[0];
          })
        },
        series: {
          type: 'bar',
          id: 'sales',
          dataGroupId: subData.dataGroupId,
          data: subData.data.map(function (item) {
            return item[1];
          }),
          universalTransition: {
            enabled: true,
            divideShape: 'clone'
          }
        },
        graphic: [
          {
            type: 'text',
            left: 50,
            top: 20,
            style: {
              text: 'Back',
              fontSize: 18
            },
            onclick: function () {
              myChart.setOption(option);
            }
          }
        ]
      });
    }
  });
  if (option) {
    myChart.setOption(option);
  }
}

main().catch((error) => {
  showPreviewError(error);
  console.error(error);
});
