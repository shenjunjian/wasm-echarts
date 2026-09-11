import { lazyNativeClass } from './native-core.js';
import * as matrix from './tool/matrix.js';

function patchBoundingRect(BoundingRect) {
  BoundingRect.calculateTransform = function (out, a, b) {
    const sx = !a.width ? 1 : b.width / a.width;
    const sy = !a.height ? 1 : b.height / a.height;
    out = matrix.identity(out || []);
    matrix.translate(out, out, [-a.x, -a.y]);
    matrix.scale(out, out, [sx, sy]);
    matrix.translate(out, out, [b.x, b.y]);
    return out;
  };
}

export const BoundingRect = lazyNativeClass('BoundingRect', patchBoundingRect);
