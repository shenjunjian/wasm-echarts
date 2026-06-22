import echarts from './js/echarts.js';

const LINE_OPTION = {
  tooltip: {
    trigger: 'item',
    axisPointer: { type: 'line' },
    formatter(params) {
      return `${params.seriesName}<br/>${params.name}: ${params.value}`;
    },
  },
  dataZoom: [{ type: 'inside', xAxisIndex: 0 }],
  xAxis: { type: 'category', data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri'] },
  yAxis: { type: 'value' },
  series: [
    {
      type: 'line',
      name: '销量',
      data: [120, 200, 150, 80, 70],
      label: {
        formatter(params) {
          return String(params.value);
        },
      },
    },
  ],
};

const BAR_OPTION = {
  tooltip: {
    trigger: 'item',
    axisPointer: { type: 'line' },
  },
  dataZoom: [{ type: 'inside', xAxisIndex: 0 }],
  xAxis: { type: 'category', data: ['A', 'B', 'C', 'D'] },
  yAxis: { type: 'value' },
  series: [{ type: 'bar', name: '数量', data: [40, 90, 60, 120] }],
};

async function main() {
  const status = document.getElementById('status');
  const container = document.getElementById('chart');

  try {
    status.textContent = '加载 WASM…';
    const chart = await echarts.init(container, { renderer: 'canvas' });

    const params = new URLSearchParams(location.search);
    const mode = params.get('type') || 'line';
    chart.setOption(mode === 'bar' ? BAR_OPTION : LINE_OPTION);

    const meta = chart.getOption();
    status.textContent = `阶段 6 Demo（${mode}）：hover 高亮 + tooltip + axisPointer；滚轮 dataZoom；点击 toggleSelect`;

    chart.on('click', ({ hit }) => {
      if (hit?.dataIndex != null) {
        status.textContent = `click toggleSelect → seriesIndex=${hit.seriesIndex}, dataIndex=${hit.dataIndex}`;
      }
    });
  } catch (err) {
    status.textContent = `错误: ${err}`;
    console.error(err);
  }
}

main();
