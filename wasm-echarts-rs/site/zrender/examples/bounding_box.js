import initWasm, { init, Group, Circle, Rect } from '@wasm-zrender/wasm_zrender.js';

async function main() {
  await initWasm();

  const canvas = document.getElementById('canvas');
  const dpr = window.devicePixelRatio || 1;
  const width = window.innerWidth;
  const height = window.innerHeight;
  canvas.width = Math.floor(width * dpr);
  canvas.height = Math.floor(height * dpr);
  canvas.style.width = `${width}px`;
  canvas.style.height = `${height}px`;
  canvas.style.touchAction = 'none';

  const zr = init(null, { width, height, devicePixelRatio: dpr });
  const w = zr.getWidth();
  const h = zr.getHeight();

  const elementStyle = {
    stroke: '#ccc',
    fill: 'white',
  };

  const group = new Group();
  const circles = [];
  for (let i = 0; i < 10; ++i) {
    const r = 50 * Math.random() + 20;
    const circle = new Circle({
      shape: {
        cx: 0,
        cy: 0,
        r,
      },
      position: [
        (w * 0.6 - r * 2) * Math.random() + r + w * 0.2,
        (h * 0.6 - r * 2) * Math.random() + r + h * 0.2,
      ],
      style: elementStyle,
      draggable: true,
    });
    group.add(circle);
    circles.push(circle);
  }
  zr.add(group);

  const rect = group.getBoundingRect();
  const boundingRect = new Rect({
    shape: {
      x: rect.x,
      y: rect.y,
      width: rect.width,
      height: rect.height,
    },
    style: {
      fill: 'none',
      stroke: '#14f1ff',
    },
    silent: true,
  });
  zr.add(boundingRect);

  paint(zr, canvas);

  // wasm-zrender 不绑定 DOM：zr.on / el.on / draggable 为空操作。
  // 拖动由宿主指针事件 + findHover + position 完成，并实时刷新蓝色包围盒。
  let dragging = null;

  function canvasXY(e) {
    const box = canvas.getBoundingClientRect();
    const scaleX = zr.getWidth() / box.width;
    const scaleY = zr.getHeight() / box.height;
    return [(e.clientX - box.left) * scaleX, (e.clientY - box.top) * scaleY];
  }

  function circleAt(x, y) {
    const hover = zr.findHover(x, y);
    if (!hover?.target) return null;
    return circles.find((c) => c.id === hover.target.id) ?? null;
  }

  function syncBoundingRect() {
    const next = group.getBoundingRect();
    boundingRect.setShape({
      x: next.x,
      y: next.y,
      width: next.width,
      height: next.height,
    });
  }

  canvas.addEventListener('pointerdown', (e) => {
    const [x, y] = canvasXY(e);
    const circle = circleAt(x, y);
    if (!circle) return;
    dragging = { circle, x, y };
    canvas.setPointerCapture(e.pointerId);
    canvas.style.cursor = 'grabbing';
  });

  canvas.addEventListener('pointermove', (e) => {
    const [x, y] = canvasXY(e);
    if (!dragging) {
      canvas.style.cursor = circleAt(x, y) ? 'move' : 'default';
      return;
    }
    const dx = x - dragging.x;
    const dy = y - dragging.y;
    dragging.x = x;
    dragging.y = y;
    const pos = dragging.circle.position;
    dragging.circle.position = [pos[0] + dx, pos[1] + dy];
    syncBoundingRect();
    paint(zr, canvas);
  });

  function endDrag() {
    dragging = null;
  }

  canvas.addEventListener('pointerup', endDrag);
  canvas.addEventListener('pointercancel', endDrag);
}

function paint(zr, canvas) {
  const rgba = zr.refresh();
  const ctx = canvas.getContext('2d');
  ctx.putImageData(
    new ImageData(new Uint8ClampedArray(rgba), zr.width(), zr.height()),
    0,
    0,
  );
}

main();
