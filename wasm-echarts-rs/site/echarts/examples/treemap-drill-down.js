/**
 * 官网示例：ECharts 配置项查询分布
 * https://echarts.apache.org/examples/zh/editor.html?c=treemap-drill-down
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: ECharts Option Query
    category: treemap
    titleCN: ECharts 配置项查询分布
    */
    const uploadedDataURL =
      ROOT_PATH + '/data/asset/data/ec-option-doc-statistics-201604.json';
    myChart.showLoading();
    $.getJSON(uploadedDataURL, function (rawData) {
      myChart.hideLoading();
      function convert(source, target, basePath) {
        for (let key in source) {
          let path = basePath ? basePath + '.' + key : key;
          if (!key.match(/^\$/)) {
            target.children = target.children || [];
            const child = {
              name: path
            };
            target.children.push(child);
            convert(source[key], child, path);
          }
        }
        if (!target.children) {
          target.value = source.$count || 1;
        } else {
          target.children.push({
            name: basePath,
            value: source.$count
          });
        }
      }
      const data = {
        children: []
      };
      convert(rawData, data, '');
      myChart.setOption(
        (option = {
          title: {
            text: 'ECharts Options',
            subtext: '2016/04',
            left: 'leafDepth'
          },
          tooltip: {},
          series: [
            {
              name: 'option',
              type: 'treemap',
              visibleMin: 300,
              data: data.children,
              leafDepth: 2,
              levels: [
                {
                  itemStyle: {
                    borderColor: '#555',
                    borderWidth: 4,
                    gapWidth: 4
                  }
                },
                {
                  colorSaturation: [0.3, 0.6],
                  itemStyle: {
                    borderColorSaturation: 0.7,
                    gapWidth: 2,
                    borderWidth: 2
                  }
                },
                {
                  colorSaturation: [0.3, 0.5],
                  itemStyle: {
                    borderColorSaturation: 0.6,
                    gapWidth: 1
                  }
                },
                {
                  colorSaturation: [0.3, 0.5]
                }
              ]
            }
          ]
        })
      );
    });
    return option;
  } catch (error) {
    if (option) {
      try {
        myChart.setOption(option);
      } catch {
        // 保留原始错误
      }
    }
    throw error;
  }
});
