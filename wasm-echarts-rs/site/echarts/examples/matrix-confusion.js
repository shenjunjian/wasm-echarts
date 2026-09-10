/**
 * 官网示例：混淆矩阵
 * https://echarts.apache.org/examples/zh/editor.html?c=matrix-confusion
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
  title: Confusion Matrix
  category: matrix
  titleCN: 混淆矩阵
  difficulty: 3
  since: 6.0.0
  */
  const label = {
    fontSize: 16,
    color: '#555'
  };
  option = {
    matrix: {
      x: {
        data: ['Positive', 'Negative'],
        label
      },
      y: {
        data: ['Positive', 'Negative'],
        label
      },
      top: 80,
      width: 600,
      left: 'center'
    },
    series: {
      type: 'custom',
      coordinateSystem: 'matrix',
      data: [
        ['Positive', 'Positive', 10],
        ['Positive', 'Negative', 2],
        ['Negative', 'Positive', 3],
        ['Negative', 'Negative', 5]
      ],
      label: {
        show: true,
        formatter: (params) => {
          const value = params.value[2];
          return (
            '{name|' +
            (params.value[0] === params.value[1] ? 'True ' : 'False ') +
            params.value[1] +
            '}\n{value|' +
            value +
            '}'
          );
        },
        rich: {
          name: {
            color: '#fff',
            backgroundColor: '#999',
            textBorderColor: '#333',
            padding: 5,
            fontSize: 18
          },
          value: {
            color: '#444',
            textBorderWidth: 0,
            padding: 5,
            fontSize: 16,
            align: 'center'
          }
        }
      },
      renderItem: function (params, api) {
        const x = api.value(0);
        const y = api.value(1);
        const rect = api.layout([x, y]).rect;
        return {
          type: 'rect',
          shape: {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height
          },
          style: api.style({
            fill: x === y ? '#8f8' : '#f88'
          })
        };
      }
    },
    graphic: {
      elements: [
        {
          type: 'text',
          style: {
            text: 'True Class',
            fill: '#333',
            font: 'bold 24px serif',
            textAlign: 'center'
          },
          x: (window.innerWidth - 600) / 2 + (600 / 6) * 4,
          y: 40
        },
        {
          type: 'text',
          style: {
            text: 'Predicted Class',
            fill: '#333',
            font: 'bold 24px serif',
            textAlign: 'center'
          },
          x: (window.innerWidth - 600) / 2 - 50,
          y: 270,
          rotation: Math.PI / 2
        }
      ]
    }
  };
  if (option) {
    myChart.setOption(option);
  }
}

main().catch((error) => {
  showPreviewError(error);
  console.error(error);
});
