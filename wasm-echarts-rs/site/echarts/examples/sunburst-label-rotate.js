/**
 * 官网示例：旭日图标签旋转
 * https://echarts.apache.org/examples/zh/editor.html?c=sunburst-label-rotate
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
  title: Sunburst Label Rotate
  category: sunburst
  titleCN: 旭日图标签旋转
  difficulty: 2
  */
  option = {
    silent: true,
    series: [
      {
        radius: ['15%', '80%'],
        type: 'sunburst',
        sort: undefined,
        emphasis: {
          focus: 'ancestor'
        },
        data: [
          {
            value: 8,
            children: [
              {
                value: 4,
                children: [
                  {
                    value: 2
                  },
                  {
                    value: 1
                  },
                  {
                    value: 1
                  },
                  {
                    value: 0.5
                  }
                ]
              },
              {
                value: 2
              }
            ]
          },
          {
            value: 4,
            children: [
              {
                children: [
                  {
                    value: 2
                  }
                ]
              }
            ]
          },
          {
            value: 4,
            children: [
              {
                children: [
                  {
                    value: 2
                  }
                ]
              }
            ]
          },
          {
            value: 3,
            children: [
              {
                children: [
                  {
                    value: 1
                  }
                ]
              }
            ]
          }
        ],
        label: {
          color: '#000',
          textBorderColor: '#fff',
          textBorderWidth: 2,
          formatter: function (param) {
            var depth = param.treePathInfo.length;
            if (depth === 2) {
              return 'radial';
            } else if (depth === 3) {
              return 'tangential';
            } else if (depth === 4) {
              return '0';
            }
            return '';
          }
        },
        levels: [
          {},
          {
            itemStyle: {
              color: '#CD4949'
            },
            label: {
              rotate: 'radial'
            }
          },
          {
            itemStyle: {
              color: '#F47251'
            },
            label: {
              rotate: 'tangential'
            }
          },
          {
            itemStyle: {
              color: '#FFC75F'
            },
            label: {
              rotate: 0
            }
          }
        ]
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
