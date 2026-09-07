import echarts from '../../src/echarts/echarts.js';

async function main() {
  const chart = await echarts.init(document.getElementById('chart'), {
    width: 480,
    height: 360,
  });

  chart.setOption({
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
  });
}

main();
