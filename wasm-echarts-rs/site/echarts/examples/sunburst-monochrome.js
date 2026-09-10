/**
 * 官网示例：单色旭日图
 * https://echarts.apache.org/examples/zh/editor.html?c=sunburst-monochrome
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Monochrome Sunburst
    category: sunburst
    titleCN: 单色旭日图
    difficulty: 3
    */
    const item1 = {
      color: '#F54F4A'
    };
    const item2 = {
      color: '#FF8C75'
    };
    const item3 = {
      color: '#FFB499'
    };
    const data = [
      {
        children: [
          {
            value: 5,
            children: [
              {
                value: 1,
                itemStyle: item1
              },
              {
                value: 2,
                children: [
                  {
                    value: 1,
                    itemStyle: item2
                  }
                ]
              },
              {
                children: [
                  {
                    value: 1
                  }
                ]
              }
            ],
            itemStyle: item1
          },
          {
            value: 10,
            children: [
              {
                value: 6,
                children: [
                  {
                    value: 1,
                    itemStyle: item1
                  },
                  {
                    value: 1
                  },
                  {
                    value: 1,
                    itemStyle: item2
                  },
                  {
                    value: 1
                  }
                ],
                itemStyle: item3
              },
              {
                value: 2,
                children: [
                  {
                    value: 1
                  }
                ],
                itemStyle: item3
              },
              {
                children: [
                  {
                    value: 1,
                    itemStyle: item2
                  }
                ]
              }
            ],
            itemStyle: item1
          }
        ],
        itemStyle: item1
      },
      {
        value: 9,
        children: [
          {
            value: 4,
            children: [
              {
                value: 2,
                itemStyle: item2
              },
              {
                children: [
                  {
                    value: 1,
                    itemStyle: item1
                  }
                ]
              }
            ],
            itemStyle: item1
          },
          {
            children: [
              {
                value: 3,
                children: [
                  {
                    value: 1
                  },
                  {
                    value: 1,
                    itemStyle: item2
                  }
                ]
              }
            ],
            itemStyle: item3
          }
        ],
        itemStyle: item2
      },
      {
        value: 7,
        children: [
          {
            children: [
              {
                value: 1,
                itemStyle: item3
              },
              {
                value: 3,
                children: [
                  {
                    value: 1,
                    itemStyle: item2
                  },
                  {
                    value: 1
                  }
                ],
                itemStyle: item2
              },
              {
                value: 2,
                children: [
                  {
                    value: 1
                  },
                  {
                    value: 1,
                    itemStyle: item1
                  }
                ],
                itemStyle: item1
              }
            ],
            itemStyle: item3
          }
        ],
        itemStyle: item1
      },
      {
        children: [
          {
            value: 6,
            children: [
              {
                value: 1,
                itemStyle: item2
              },
              {
                value: 2,
                children: [
                  {
                    value: 2,
                    itemStyle: item2
                  }
                ],
                itemStyle: item1
              },
              {
                value: 1,
                itemStyle: item3
              }
            ],
            itemStyle: item3
          },
          {
            value: 3,
            children: [
              {
                value: 1
              },
              {
                children: [
                  {
                    value: 1,
                    itemStyle: item2
                  }
                ]
              },
              {
                value: 1
              }
            ],
            itemStyle: item3
          }
        ],
        itemStyle: item1
      }
    ];
    option = {
      series: {
        radius: ['15%', '80%'],
        type: 'sunburst',
        sort: undefined,
        emphasis: {
          focus: 'ancestor'
        },
        data: data,
        label: {
          rotate: 'radial'
        },
        levels: [],
        itemStyle: {
          color: '#ddd',
          borderWidth: 2
        }
      }
    };
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
