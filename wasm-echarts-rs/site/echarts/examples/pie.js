import initWasm, { EChartsInstance } from '@wasm-echarts';

const width = 480;
const height = 360;

async function main() {
  await initWasm();

  const canvas = document.getElementById('canvas');
  canvas.width = width;
  canvas.height = height;
  canvas.style.width = `${width}px`;
  canvas.style.height = `${height}px`;

  const chart = new EChartsInstance(width, height, 1);
  chart.set_option({
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

  paint(chart, canvas);
}

function paint(chart, canvas) {
  const rgba = chart.refresh();
  const ctx = canvas.getContext('2d');
  ctx.putImageData(
    new ImageData(new Uint8ClampedArray(rgba), chart.width(), chart.height()),
    0,
    0,
  );
}

main();
