/**
 * 官网示例：旭日图使用视觉编码
 * https://echarts.apache.org/examples/zh/editor.html?c=sunburst-visualMap
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
  title: Sunburst VisualMap
  category: sunburst
  titleCN: 旭日图使用视觉编码
  difficulty: 4
  */
  var data = [
    {
      name: 'Grandpa',
      children: [
        {
          name: 'Uncle Leo',
          value: 15,
          children: [
            {
              name: 'Cousin Jack',
              value: 2
            },
            {
              name: 'Cousin Mary',
              value: 5,
              children: [
                {
                  name: 'Jackson',
                  value: 2
                }
              ]
            },
            {
              name: 'Cousin Ben',
              value: 4
            }
          ]
        },
        {
          name: 'Aunt Jane',
          children: [
            {
              name: 'Cousin Kate',
              value: 4
            }
          ]
        },
        {
          name: 'Father',
          value: 10,
          children: [
            {
              name: 'Me',
              value: 5,
              itemStyle: {
                color: 'red'
              }
            },
            {
              name: 'Brother Peter',
              value: 1
            }
          ]
        }
      ]
    },
    {
      name: 'Mike',
      children: [
        {
          name: 'Uncle Dan',
          children: [
            {
              name: 'Cousin Lucy',
              value: 3
            },
            {
              name: 'Cousin Luck',
              value: 4,
              children: [
                {
                  name: 'Nephew',
                  value: 2
                }
              ]
            }
          ]
        }
      ]
    },
    {
      name: 'Nancy',
      children: [
        {
          name: 'Uncle Nike',
          children: [
            {
              name: 'Cousin Betty',
              value: 1
            },
            {
              name: 'Cousin Jenny',
              value: 2
            }
          ]
        }
      ]
    }
  ];
  option = {
    visualMap: {
      type: 'continuous',
      min: 0,
      max: 10,
      inRange: {
        color: ['#2F93C8', '#AEC48F', '#FFDB5C', '#F98862']
      }
    },
    series: {
      type: 'sunburst',
      data: data,
      radius: [0, '90%'],
      label: {
        rotate: 'radial'
      }
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
