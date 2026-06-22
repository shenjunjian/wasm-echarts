import echarts from './js/echarts.js';

const DEFAULT_OPTION = {
  xAxis: { type: 'category', data: ['Mon', 'Tue', 'Wed'] },
  yAxis: { type: 'value' },
  series: [
    {
      type: 'line',
      name: 'demo',
      data: [120, 200, 150],
      label: {
        formatter(params) {
          return `值: ${params.value}`;
        },
      },
    },
  ],
};

async function main() {
  const status = document.getElementById('status');
  const container = document.getElementById('chart');

  try {
    status.textContent = '加载 WASM…';

    const chart = await echarts.init(container, { renderer: 'canvas' });

    chart.setOption(DEFAULT_OPTION);

    const meta = chart.getOption();
    status.textContent = `阶段 4 Demo：option 已合并（含函数: ${meta.hasFunctions}）`;

    chart.on('click', ({ hit }) => {
      if (hit) {
        status.textContent = `click → seriesIndex=${hit.seriesIndex ?? '-'}, dataIndex=${hit.dataIndex ?? '-'}, pathIndex=${hit.pathIndex}`;
      }
    });

    chart.on('mouseover', ({ hit }) => {
      if (hit?.dataIndex != null) {
        container.title = `dataIndex: ${hit.dataIndex}`;
      }
    });
  } catch (err) {
    status.textContent = `错误: ${err}`;
    console.error(err);
  }
}

main();
