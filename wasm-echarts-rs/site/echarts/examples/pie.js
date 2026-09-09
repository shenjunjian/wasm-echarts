import initWasm, { init } from '@wasm-echarts';

const width = 480;
const height = 360;

async function main() {
  await initWasm();

  const canvas = document.getElementById('canvas');
  canvas.width = width;
  canvas.height = height;
  canvas.style.width = `${width}px`;
  canvas.style.height = `${height}px`;

  const chart = init(canvas);
  chart.setOption({
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
