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
    },
  ],
};

const BAR_OPTION = {
  tooltip: { trigger: 'item', axisPointer: { type: 'line' } },
  dataZoom: [{ type: 'inside', xAxisIndex: 0 }],
  xAxis: { type: 'category', data: ['A', 'B', 'C', 'D'] },
  yAxis: { type: 'value' },
  series: [{ type: 'bar', name: '数量', data: [40, 90, 60, 120] }],
};

const PIE_OPTION = {
  tooltip: { trigger: 'item' },
  series: [
    {
      type: 'pie',
      name: '占比',
      radius: '55%',
      data: [
        { name: '直接访问', value: 335 },
        { name: '邮件营销', value: 310 },
        { name: '联盟广告', value: 234 },
        { name: '视频广告', value: 135 },
        { name: '搜索引擎', value: 1548 },
      ],
    },
  ],
};

const SCATTER_OPTION = {
  tooltip: { trigger: 'item' },
  xAxis: { type: 'value', scale: true },
  yAxis: { type: 'value', scale: true },
  series: [
    {
      type: 'scatter',
      name: '样本',
      data: [
        [10.0, 8.04],
        [8.07, 6.95],
        [13.0, 7.58],
        [9.05, 8.81],
        [11.0, 8.33],
        [14.0, 7.66],
        [13.4, 6.81],
        [10.0, 6.33],
        [14.0, 8.96],
        [12.5, 6.82],
      ],
    },
  ],
};

const OPTIONS = {
  line: LINE_OPTION,
  bar: BAR_OPTION,
  pie: PIE_OPTION,
  scatter: SCATTER_OPTION,
};

async function main() {
  const status = document.getElementById('status');
  const container = document.getElementById('chart');

  try {
    status.textContent = '加载 WASM…';
    const chart = await echarts.init(container, { renderer: 'canvas' });

    const params = new URLSearchParams(location.search);
    const mode = params.get('type') || 'line';
    const option = OPTIONS[mode] ?? LINE_OPTION;
    chart.setOption(option);

    if (params.get('bench') === '1') {
      const ms = chart.benchmark(30);
      status.textContent = `阶段 7 基准（${mode}）：render+refresh 均值 ${ms.toFixed(2)} ms（30 次）`;
    } else {
      status.textContent = `阶段 7 Demo（${mode}）：line/bar/pie/scatter + 轴标签；?bench=1 跑基准`;
    }

    chart.on('click', ({ hit }) => {
      if (hit?.dataIndex != null) {
        status.textContent = `click → seriesIndex=${hit.seriesIndex}, dataIndex=${hit.dataIndex}`;
      }
    });
  } catch (err) {
    status.textContent = `错误: ${err}`;
    console.error(err);
  }
}

main();
