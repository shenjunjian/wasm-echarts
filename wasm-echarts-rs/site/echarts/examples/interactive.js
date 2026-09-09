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
    series: [{ type: 'line', name: '销量', data: [120, 200, 150, 80, 70] }],
  });

  paint(chart, canvas);

  const logEl = document.getElementById('log');
  const tip = document.createElement('div');
  tip.style.cssText =
    'position:fixed;display:none;padding:6px 10px;background:rgba(50,50,50,0.9);color:#fff;font:12px/1.4 system-ui,sans-serif;border-radius:4px;pointer-events:none;white-space:nowrap;z-index:10;';
  document.body.appendChild(tip);

  canvas.addEventListener('mousemove', (e) => {
    const rect = canvas.getBoundingClientRect();
    const result = chart.handle_pointer_move(
      e.clientX - rect.left,
      e.clientY - rect.top,
    );
    paint(chart, canvas);

    if (result?.tooltip) {
      tip.innerHTML = result.tooltip;
      tip.style.display = 'block';
      tip.style.left = `${e.clientX + 12}px`;
      tip.style.top = `${e.clientY + 12}px`;
    } else {
      tip.style.display = 'none';
    }
  });

  canvas.addEventListener('mouseleave', () => {
    chart.handle_pointer_leave();
    paint(chart, canvas);
    tip.style.display = 'none';
  });

  canvas.addEventListener('click', (e) => {
    const rect = canvas.getBoundingClientRect();
    const hit = chart.find_hover(e.clientX - rect.left, e.clientY - rect.top);
    if (hit?.seriesIndex == null || hit?.dataIndex == null) return;

    chart.dispatch_action({
      type: 'toggleSelect',
      seriesIndex: hit.seriesIndex,
      dataIndex: hit.dataIndex,
    });
    paint(chart, canvas);
    if (logEl) {
      logEl.textContent = `click select → seriesIndex=${hit.seriesIndex}, dataIndex=${hit.dataIndex}`;
    }
  });

  canvas.addEventListener(
    'wheel',
    (e) => {
      e.preventDefault();
      const rect = canvas.getBoundingClientRect();
      chart.apply_data_zoom_wheel(e.clientX - rect.left, e.deltaY);
      paint(chart, canvas);
    },
    { passive: false },
  );
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
