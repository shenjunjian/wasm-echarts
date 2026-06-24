/**
 * 各 zrender 示例的场景构建（与 examples-catalog 展示源码一致）
 */

import {
  Group,
  Rect,
  Circle,
  Line,
  Polygon,
  Sector,
  Text,
  LinearGradient,
} from '@wasm-zrender/wasm_zrender.js';

const COLORS = ['#5470c6', '#91cc75', '#fac858', '#ee6666', '#73c0de', '#3ba272'];

/** @param {import('@wasm-zrender/wasm_zrender.js').ZRender} zr */
export function setupShapes(zr) {
  const g = new Group();

  g.add(
    new Rect({
      shape: { x: 20, y: 20, width: 100, height: 60 },
      style: {
        fill: new LinearGradient(0, 0, 1, 0, [
          { offset: 0, color: '#5470c6' },
          { offset: 1, color: '#91cc75' },
        ]),
      },
    }),
  );

  g.add(
    new Circle({
      shape: { cx: 180, cy: 80, r: 40 },
      style: {
        fill: 'rgba(145, 204, 117, 0.8)',
        stroke: '#ee6666',
        lineWidth: 3,
      },
      z: 1,
    }),
  );

  g.add(
    new Line({
      shape: { x1: 20, y1: 120, x2: 280, y2: 120 },
      style: {
        stroke: '#333',
        lineWidth: 2,
        lineDash: [6, 4],
      },
    }),
  );

  g.add(
    new Polygon({
      shape: {
        points: [
          [240, 30],
          [300, 60],
          [270, 100],
        ],
      },
      style: { fill: '#fac858' },
    }),
  );

  zr.add(g);
}

/** @param {import('@wasm-zrender/wasm_zrender.js').ZRender} zr */
export function setupText(zr) {
  zr.add(
    new Text({
      style: {
        text: 'wasm-zrender 文本',
        x: 24,
        y: 48,
        fill: '#333',
        fontSize: 18,
        fontWeight: 'bold',
      },
    }),
  );

  zr.add(
    new Text({
      style: {
        text: '对齐 · 中文 · fillText',
        x: 24,
        y: 96,
        fill: '#5470c6',
        fontSize: 14,
      },
    }),
  );

  zr.add(
    new Text({
      style: {
        text: 'right align',
        x: 440,
        y: 140,
        fill: '#666',
        fontSize: 12,
        textAlign: 'right',
      },
    }),
  );
}

/** @param {import('@wasm-zrender/wasm_zrender.js').ZRender} zr */
export function setupSector(zr) {
  const g = new Group();
  const cx = 240;
  const cy = 180;
  const r = 120;
  const values = [30, 70, 100, 50];
  const total = values.reduce((sum, v) => sum + v, 0);
  let angle = -Math.PI / 2;

  values.forEach((value, i) => {
    const sweep = (value / total) * Math.PI * 2;
    const start = angle;
    const end = angle + sweep;
    angle = end;

    g.add(
      new Sector({
        shape: { cx, cy, r, startAngle: start, endAngle: end },
        style: {
          fill: COLORS[i % COLORS.length],
          stroke: '#fff',
          lineWidth: 1,
        },
        seriesIndex: 0,
        dataIndex: i,
      }),
    );
  });

  zr.add(g);
}

/** @param {import('@wasm-zrender/wasm_zrender.js').ZRender} zr @returns {import('@wasm-zrender/wasm_zrender.js').Rect[]} */
export function setupHit(zr) {
  const g = new Group();
  const rects = [];

  const items = [
    { x: 40, y: 40, w: 90, h: 60, dataIndex: 0 },
    { x: 160, y: 50, w: 100, h: 70, dataIndex: 1 },
    { x: 300, y: 60, w: 80, h: 80, dataIndex: 2 },
  ];

  items.forEach((item, i) => {
    const rect = new Rect({
      shape: { x: item.x, y: item.y, width: item.w, height: item.h },
      style: { fill: COLORS[i] },
      seriesIndex: 0,
      dataIndex: item.dataIndex,
    });
    rects.push(rect);
    g.add(rect);
  });

  zr.add(g);
  return rects;
}

/** @param {import('@wasm-zrender/wasm_zrender.js').ZRender} zr @returns {import('@wasm-zrender/wasm_zrender.js').Rect[]} */
export function setupState(zr) {
  const g = new Group();
  const rects = [];

  for (let row = 0; row < 2; row++) {
    for (let col = 0; col < 3; col++) {
      const i = row * 3 + col;
      const rect = new Rect({
        shape: { x: 40 + col * 130, y: 60 + row * 100, width: 100, height: 70 },
        style: { fill: COLORS[i] },
        seriesIndex: 0,
        dataIndex: i,
      });
      rect.setStateStyle('emphasis', { fill: '#ee6666', lineWidth: 4 });
      rects.push(rect);
      g.add(rect);
    }
  }

  zr.add(g);
  return rects;
}

export const ZRENDER_SETUPS = {
  shapes: setupShapes,
  text: setupText,
  sector: setupSector,
  hit: setupHit,
  state: setupState,
};
