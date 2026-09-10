/**
 * 官网示例：自定义仪表
 * https://echarts.apache.org/examples/zh/editor.html?c=custom-gauge
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
  title: Custom Gauge
  category: custom
  titleCN: 自定义仪表
  difficulty: 9
  */
  var _panelImageURL = ROOT_PATH + '/data/asset/img/custom-gauge-panel.png';
  var _animationDuration = 1000;
  var _animationDurationUpdate = 1000;
  var _animationEasingUpdate = 'quarticInOut';
  var _valOnRadianMax = 200;
  var _outerRadius = 200;
  var _innerRadius = 170;
  var _pointerInnerRadius = 40;
  var _insidePanelRadius = 140;
  var _currentDataIndex = 0;
  function renderItem(params, api) {
    var valOnRadian = api.value(1);
    var coords = api.coord([api.value(0), valOnRadian]);
    var polarEndRadian = coords[3];
    var imageStyle = {
      image: _panelImageURL,
      x: params.coordSys.cx - _outerRadius,
      y: params.coordSys.cy - _outerRadius,
      width: _outerRadius * 2,
      height: _outerRadius * 2
    };
    return {
      type: 'group',
      children: [
        {
          type: 'image',
          style: imageStyle,
          clipPath: {
            type: 'sector',
            shape: {
              cx: params.coordSys.cx,
              cy: params.coordSys.cy,
              r: _outerRadius,
              r0: _innerRadius,
              startAngle: 0,
              endAngle: -polarEndRadian,
              transition: 'endAngle',
              enterFrom: { endAngle: 0 }
            }
          }
        },
        {
          type: 'image',
          style: imageStyle,
          clipPath: {
            type: 'polygon',
            shape: {
              points: makePionterPoints(params, polarEndRadian)
            },
            extra: {
              polarEndRadian: polarEndRadian,
              transition: 'polarEndRadian',
              enterFrom: { polarEndRadian: 0 }
            },
            during: function (apiDuring) {
              apiDuring.setShape(
                'points',
                makePionterPoints(params, apiDuring.getExtra('polarEndRadian'))
              );
            }
          }
        },
        {
          type: 'circle',
          shape: {
            cx: params.coordSys.cx,
            cy: params.coordSys.cy,
            r: _insidePanelRadius
          },
          style: {
            fill: '#fff',
            shadowBlur: 25,
            shadowOffsetX: 0,
            shadowOffsetY: 0,
            shadowColor: 'rgba(76,107,167,0.4)'
          }
        },
        {
          type: 'text',
          extra: {
            valOnRadian: valOnRadian,
            transition: 'valOnRadian',
            enterFrom: { valOnRadian: 0 }
          },
          style: {
            text: makeText(valOnRadian),
            fontSize: 50,
            fontWeight: 700,
            x: params.coordSys.cx,
            y: params.coordSys.cy,
            fill: 'rgb(0,50,190)',
            align: 'center',
            verticalAlign: 'middle',
            enterFrom: { opacity: 0 }
          },
          during: function (apiDuring) {
            apiDuring.setStyle(
              'text',
              makeText(apiDuring.getExtra('valOnRadian'))
            );
          }
        }
      ]
    };
  }
  function convertToPolarPoint(renderItemParams, radius, radian) {
    return [
      Math.cos(radian) * radius + renderItemParams.coordSys.cx,
      -Math.sin(radian) * radius + renderItemParams.coordSys.cy
    ];
  }
  function makePionterPoints(renderItemParams, polarEndRadian) {
    return [
      convertToPolarPoint(renderItemParams, _outerRadius, polarEndRadian),
      convertToPolarPoint(
        renderItemParams,
        _outerRadius,
        polarEndRadian + Math.PI * 0.03
      ),
      convertToPolarPoint(renderItemParams, _pointerInnerRadius, polarEndRadian)
    ];
  }
  function makeText(valOnRadian) {
    // Validate additive animation calc.
    if (valOnRadian < -10) {
      alert('illegal during val: ' + valOnRadian);
    }
    return ((valOnRadian / _valOnRadianMax) * 100).toFixed(0) + '%';
  }
  option = {
    animationEasing: _animationEasingUpdate,
    animationDuration: _animationDuration,
    animationDurationUpdate: _animationDurationUpdate,
    animationEasingUpdate: _animationEasingUpdate,
    dataset: {
      source: [[1, 156]]
    },
    tooltip: {},
    angleAxis: {
      type: 'value',
      startAngle: 0,
      show: false,
      min: 0,
      max: _valOnRadianMax
    },
    radiusAxis: {
      type: 'value',
      show: false
    },
    polar: {},
    series: [
      {
        type: 'custom',
        coordinateSystem: 'polar',
        renderItem: renderItem
      }
    ]
  };
  setInterval(function () {
    var nextSource = [[1, Math.round(Math.random() * _valOnRadianMax)]];
    myChart.setOption({
      dataset: {
        source: nextSource
      }
    });
  }, 3000);
  if (option) {
    myChart.setOption(option);
  }
}

main().catch((error) => {
  showPreviewError(error);
  console.error(error);
});
