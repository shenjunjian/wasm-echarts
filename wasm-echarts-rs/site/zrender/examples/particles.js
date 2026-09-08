import initWasm, { init, Circle } from '@wasm-zrender';

async function main() {
  await initWasm();

  const canvas = document.getElementById('canvas');
  const width = window.innerWidth;
  const height = window.innerHeight;
  canvas.width = width;
  canvas.height = height;
  canvas.style.width = `${width}px`;
  canvas.style.height = `${height}px`;

  const zr = init(canvas, { width, height });
  const w = zr.getWidth();
  const h = zr.getHeight();

  zr.animation.on('frame', () => {
    console.log("on frame")
  });

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
          const x1 = p.position[0] + vx * dt* .5;
          const y1 = p.position[1] + vy * dt* .5 ;
          p.position = [x1, y1];
          p.setStyle({
            opacity: 0.5,
          });
          t0 = _t;
          void ay;
        })
        .done(() => {
          console.log('particle removed');
        })
        .start();
    }
  };

  spray(w / 2, h / 2);
  // paint(zr, canvas);
  // zr.flush();
}

main();
