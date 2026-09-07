import initWasm, { init, Circle } from '@wasm-zrender/wasm_zrender.js';

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

  const zr = init(null, { width, height, devicePixelRatio: dpr });
  const w = zr.getWidth();
  const h = zr.getHeight();

  zr.animation.on('frame', () => {});

  const spray = (x, y) => {
    const cnt = 200;
    const centerTolerance = 0;
    const radius = 10;
    const duration = 3000;
    const color = Math.random() * 260;
    const maxVx = 1000 + Math.random() * 1500;
    const maxVy = 1000 + Math.random() * 1500;

    for (let i = 0; i < cnt; ++i) {
      const x0 = x + centerTolerance * (Math.random() - 1);
      const y0 = y + centerTolerance * (Math.random() - 1);
      const opacity = Math.random() * 0.5 + 0.5;

      const particle = new Circle({
        shape: {
          cx: 0,
          cy: 0,
          r: radius * (0.5 + 0.5 * Math.random()),
        },
        style: {
          fill: `hsl(${Math.floor(color + Math.random() * 100)}, 80%, ${Math.floor(Math.random() * 40 + 40)}%)`,
          opacity,
        },
        position: [x0, y0],
      });
      zr.add(particle);

      particle._t = 0;
      particle._opacity = opacity;

      const animator = particle.animate('');
      const vx = (Math.random() - 0.5) * maxVx;
      const vy = (Math.random() - 1.2) * maxVy;
      const ay = 8000;
      let t0 = 0;

      animator
        .when(duration, { _t: 1 })
        .during((p, _t) => {
          const dt = _t - t0;
          const x1 = p.position[0] + vx * dt;
          const y1 = p.position[1] + vy * dt;
          p.position = [x1, y1];
          p.setStyle({
            opacity: p._opacity * (1 - _t),
          });
          t0 = _t;
          void ay;
        })
        .done(() => {
          zr.remove(particle);
        })
        .start();
    }
  };

  spray(w / 2, h / 2);
  paint(zr, canvas);
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
