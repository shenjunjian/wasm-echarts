/**
 * 官网折线示例：可拖拽点
 * https://echarts.apache.org/examples/zh/editor.html?c=line-draggable
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, $, app }) => {
  let option;
  try {
    /*
    title: Draggable Points
    category: 'line, graphic'
    titleCN: 可拖拽点
    difficulty: 12
    */
    const symbolSize = 20;
    const data = [
      [40, -10],
      [-30, -5],
      [-76.5, 20],
      [-63.5, 40],
      [-22.1, 50]
    ];
    option = {
      title: {
        text: 'Try Dragging these Points',
        left: 'center'
      },
      tooltip: {
        triggerOn: 'none',
        formatter: function (params) {
          return (
            'X: ' +
            params.data[0].toFixed(2) +
            '<br>Y: ' +
            params.data[1].toFixed(2)
          );
        }
      },
      grid: {
        top: '8%',
        bottom: '12%'
      },
      xAxis: {
        min: -100,
        max: 70,
        type: 'value',
        axisLine: { onZero: false }
      },
      yAxis: {
        min: -30,
        max: 60,
        type: 'value',
        axisLine: { onZero: false }
      },
      dataZoom: [
        {
          type: 'slider',
          xAxisIndex: 0,
          filterMode: 'none'
        },
        {
          type: 'slider',
          yAxisIndex: 0,
          filterMode: 'none'
        },
        {
          type: 'inside',
          xAxisIndex: 0,
          filterMode: 'none'
        },
        {
          type: 'inside',
          yAxisIndex: 0,
          filterMode: 'none'
        }
      ],
      series: [
        {
          id: 'a',
          type: 'line',
          smooth: true,
          symbolSize: symbolSize,
          data: data
        }
      ]
    };
    setTimeout(function () {
      // Add shadow circles (which is not visible) to enable drag.
      myChart.setOption({
        graphic: data.map(function (item, dataIndex) {
          return {
            type: 'circle',
            position: myChart.convertToPixel('grid', item),
            shape: {
              cx: 0,
              cy: 0,
              r: symbolSize / 2
            },
            invisible: true,
            draggable: true,
            ondrag: function (dx, dy) {
              onPointDragging(dataIndex, [this.x, this.y]);
            },
            onmousemove: function () {
              showTooltip(dataIndex);
            },
            onmouseout: function () {
              hideTooltip(dataIndex);
            },
            z: 100
          };
        })
      });
    }, 0);
    window.addEventListener('resize', updatePosition);
    myChart.on('dataZoom', updatePosition);
    function updatePosition() {
      myChart.setOption({
        graphic: data.map(function (item, dataIndex) {
          return {
            position: myChart.convertToPixel('grid', item)
          };
        })
      });
    }
    function showTooltip(dataIndex) {
      myChart.dispatchAction({
        type: 'showTip',
        seriesIndex: 0,
        dataIndex: dataIndex
      });
    }
    function hideTooltip(dataIndex) {
      myChart.dispatchAction({
        type: 'hideTip'
      });
    }
    function onPointDragging(dataIndex, pos) {
      data[dataIndex] = myChart.convertFromPixel('grid', pos);
      // Update data
      myChart.setOption({
        series: [
          {
            id: 'a',
            data: data
          }
        ]
      });
    }
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
