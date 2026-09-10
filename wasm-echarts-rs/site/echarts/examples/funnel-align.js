/**
 * 官网示例：漏斗图(对比)
 * https://echarts.apache.org/examples/zh/editor.html?c=funnel-align
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
  title: Funnel Compare
  category: funnel
  titleCN: 漏斗图(对比)
  */
  option = {
    title: {
      text: 'Funnel Compare',
      subtext: 'Fake Data',
      left: 'left',
      top: 'bottom'
    },
    tooltip: {
      trigger: 'item',
      formatter: '{a} <br/>{b} : {c}%'
    },
    toolbox: {
      show: true,
      orient: 'vertical',
      top: 'center',
      feature: {
        dataView: { readOnly: false },
        restore: {},
        saveAsImage: {}
      }
    },
    legend: {
      orient: 'vertical',
      left: 'left',
      data: ['Prod A', 'Prod B', 'Prod C', 'Prod D', 'Prod E']
    },
    series: [
      {
        name: 'Funnel',
        type: 'funnel',
        width: '40%',
        height: '45%',
        left: '5%',
        top: '50%',
        funnelAlign: 'right',
        data: [
          { value: 60, name: 'Prod C' },
          { value: 30, name: 'Prod D' },
          { value: 10, name: 'Prod E' },
          { value: 80, name: 'Prod B' },
          { value: 100, name: 'Prod A' }
        ]
      },
      {
        name: 'Pyramid',
        type: 'funnel',
        width: '40%',
        height: '45%',
        left: '5%',
        top: '5%',
        sort: 'ascending',
        funnelAlign: 'right',
        data: [
          { value: 60, name: 'Prod C' },
          { value: 30, name: 'Prod D' },
          { value: 10, name: 'Prod E' },
          { value: 80, name: 'Prod B' },
          { value: 100, name: 'Prod A' }
        ]
      },
      {
        name: 'Funnel',
        type: 'funnel',
        width: '40%',
        height: '45%',
        left: '55%',
        top: '5%',
        funnelAlign: 'left',
        data: [
          { value: 60, name: 'Prod C' },
          { value: 30, name: 'Prod D' },
          { value: 10, name: 'Prod E' },
          { value: 80, name: 'Prod B' },
          { value: 100, name: 'Prod A' }
        ]
      },
      {
        name: 'Pyramid',
        type: 'funnel',
        width: '40%',
        height: '45%',
        left: '55%',
        top: '50%',
        sort: 'ascending',
        funnelAlign: 'left',
        data: [
          { value: 60, name: 'Prod C' },
          { value: 30, name: 'Prod D' },
          { value: 10, name: 'Prod E' },
          { value: 80, name: 'Prod B' },
          { value: 100, name: 'Prod A' }
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
