import { BoundingRect } from './native.js';
import * as matrix from './tool/matrix.js';

BoundingRect.calculateTransform = function (out, a, b) {
  const sx = !a.width ? 1 : b.width / a.width;
  const sy = !a.height ? 1 : b.height / a.height;
  out = matrix.identity(out || []);
  matrix.translate(out, out, [-a.x, -a.y]);
  matrix.scale(out, out, [sx, sy]);
  matrix.translate(out, out, [b.x, b.y]);
  return out;
};

export { BoundingRect };
