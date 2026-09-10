/**
 * 官网示例：65k+ 飞机航线
 * https://echarts.apache.org/examples/zh/editor.html?c=lines-airline
 * 未实现的官方 API 保持报错，不在本文件里补齐。
 */
import { runOfficialExample } from '../../src/echarts/official-runtime.js';

runOfficialExample(async ({ echarts, myChart, ROOT_PATH, CDN_PATH, $, app }) => {
  let option;
  try {
    /*
    title: 65k+ Airline
    category: 'map, lines'
    titleCN: 65k+ 飞机航线
    */
    myChart.showLoading();
    $.get(ROOT_PATH + '/data/asset/data/flights.json', function (data) {
      myChart.hideLoading();
      function getAirportCoord(idx) {
        return [data.airports[idx][3], data.airports[idx][4]];
      }
      var routes = data.routes.map(function (airline) {
        return [getAirportCoord(airline[1]), getAirportCoord(airline[2])];
      });
      myChart.setOption(
        (option = {
          title: {
            text: 'World Flights',
            left: 'center',
            textStyle: {
              color: '#eee'
            }
          },
          backgroundColor: '#003',
          tooltip: {
            formatter: function (param) {
              var route = data.routes[param.dataIndex];
              return (
                data.airports[route[1]][1] + ' > ' + data.airports[route[2]][1]
              );
            }
          },
          geo: {
            map: 'world',
            left: 0,
            right: 0,
            silent: true,
            roam: true,
            itemStyle: {
              borderColor: '#003',
              color: '#005'
            }
          },
          series: [
            {
              type: 'lines',
              coordinateSystem: 'geo',
              data: routes,
              large: true,
              largeThreshold: 100,
              lineStyle: {
                opacity: 0.05,
                width: 0.5,
                curveness: 0.3
              },
              blendMode: 'lighter'
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
