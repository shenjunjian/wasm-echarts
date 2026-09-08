/**
 * 纯 JS 工具模块自检（不依赖 WASM）。
 * node js/tool/selftest.js
 */
import * as matrix from './matrix.js';
import * as vector from './vector.js';
import * as color from './color.js';
import * as util from './util.js';
import * as morph from './morph.js';
import { setPlatformAPI, platformApi } from './platform.js';
import { Animator } from '../animator.js';

let failed = 0;

function assert(cond, msg) {
  if (!cond) {
    failed += 1;
    console.error('FAIL:', msg);
  }
}

function almost(a, b, eps) {
  return Math.abs(a - b) < (eps || 1e-6);
}

{
  const m = matrix.create();
  assert(m[0] === 1 && m[3] === 1 && m[4] === 0, 'matrix.create identity');
  const t = matrix.translate([], m, [10, 20]);
  assert(t[4] === 10 && t[5] === 20, 'matrix.translate');
  const out = matrix.mul([], t, matrix.identity([]));
  assert(out[4] === 10 && out[5] === 20, 'matrix.mul');
  const inv = matrix.invert([], t);
  const roundtrip = matrix.mul([], t, inv);
  assert(almost(roundtrip[0], 1) && almost(roundtrip[4], 0), 'matrix.invert');
}

{
  const a = vector.create(3, 4);
  assert(vector.len(a) === 5, 'vector.len');
  const b = vector.add([], [1, 2], [3, 4]);
  assert(b[0] === 4 && b[1] === 6, 'vector.add');
  const p = vector.applyTransform([], [1, 1], [1, 0, 0, 1, 10, 20]);
  assert(p[0] === 11 && p[1] === 21, 'vector.applyTransform');
}

{
  const red = color.parse('#f00');
  assert(red[0] === 255 && red[1] === 0 && red[2] === 0 && red[3] === 1, 'color.parse #f00');
  const named = color.parse('red');
  assert(named[0] === 255 && named[2] === 0, 'color.parse named');
  assert(color.toHex('#ff0000') === 'ff0000', 'color.toHex');
  const mid = color.lerp(0.5, ['#000', '#fff']);
  assert(typeof mid === 'string' && mid.indexOf('rgba') === 0, 'color.lerp');
  const lifted = color.lift('#808080', -0.1);
  assert(typeof lifted === 'string', 'color.lift');
}

{
  const merged = util.merge({ a: 1, nested: { x: 1 } }, { b: 2, nested: { y: 2 } }, true);
  assert(merged.a === 1 && merged.b === 2 && merged.nested.x === 1 && merged.nested.y === 2, 'util.merge');
  const cloned = util.clone({ a: [1, 2] });
  cloned.a[0] = 9;
  assert(cloned.a[0] === 9, 'util.clone mutates copy');
  const map = util.createHashMap({ foo: 1 });
  assert(map.get('foo') === 1 && map.hasKey('foo'), 'util.HashMap');
  assert(util.guid() > 0, 'util.guid');
  assert(util.normalizeCssArray(3).join(',') === '3,3,3,3', 'util.normalizeCssArray');
}

{
  const to = { id: 'to' };
  let done = false;
  const result = morph.morphPath({ id: 'from' }, to, {
    done() {
      done = true;
    },
  });
  assert(result === to && done, 'morph.morphPath returns end path');
  const c = morph.centroid([0, 0, 2, 0, 2, 2, 0, 2]);
  assert(almost(c[0], 1) && almost(c[1], 1), 'morph.centroid');
}

{
  let called = false;
  setPlatformAPI({
    getTime() {
      called = true;
      return 42;
    },
  });
  assert(platformApi.getTime() === 42 && called, 'setPlatformAPI');
}

{
  const el = {
    shape: { cx: 10 },
    setShape(patch) {
      Object.assign(this.shape, patch);
    },
    attr(obj) {
      Object.assign(this, obj);
    },
  };
  let duringPct = null;
  let done = false;
  new Animator(el, 'shape', true)
    .when(500, { cx: 50 })
    .when(1000, { cx: 90 })
    .during((_target, pct) => {
      duringPct = pct;
    })
    .done(() => {
      done = true;
    })
    .start();
  assert(el.shape.cx === 90, 'Animator writes last when');
  assert(duringPct === 1 && done, 'Animator during/done at end');
}

if (failed) {
  console.error(failed + ' assertion(s) failed');
  process.exit(1);
}
console.log('js/tool selftest ok');
