/**
 * 回归：Vite 生产分包会先求值 bounding_rect / point 分块，
 * 再求值带 setNative 的 echarts / zrender native.js。
 * 顶层读 native.Xxx 会在 GitHub Pages 上直接炸。
 */
import assert from 'node:assert/strict';
import { test } from 'node:test';
import { BoundingRect } from './bounding_rect.js';
import { Point } from './point.js';
import { setNative } from './native-core.js';

test('facade 模块可在 setNative 之前求值', () => {
  assert.equal(typeof BoundingRect, 'function');
  assert.equal(typeof Point, 'function');
});

test('注入前构造仍提示 native 未注入', () => {
  assert.throws(() => new BoundingRect(0, 0, 1, 1), /native 未注入/);
  assert.throws(() => new Point(), /native 未注入/);
});

test('注入后可构造，并保留静态方法补丁', () => {
  class FakeRect {
    constructor(x, y, width, height) {
      this.x = x;
      this.y = y;
      this.width = width;
      this.height = height;
    }
  }
  class FakePoint {
    constructor(x = 0, y = 0) {
      this.x = x;
      this.y = y;
    }
  }

  setNative({ BoundingRect: FakeRect, Point: FakePoint });

  const rect = new BoundingRect(1, 2, 3, 4);
  assert.equal(rect.x, 1);
  assert.equal(rect.width, 3);
  assert.equal(typeof BoundingRect.calculateTransform, 'function');

  const out = BoundingRect.calculateTransform(
    null,
    { x: 0, y: 0, width: 10, height: 20 },
    { x: 5, y: 6, width: 20, height: 40 },
  );
  assert.ok(Array.isArray(out));
  assert.equal(out[0], 2);
  assert.equal(out[3], 2);

  const point = new Point(3, 4);
  assert.equal(typeof Point.len, 'function');
  assert.equal(Point.len(point), 5);
});
