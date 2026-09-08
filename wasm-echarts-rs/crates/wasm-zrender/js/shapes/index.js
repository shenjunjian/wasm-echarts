import { Path } from '../path.js';
import { native } from '../native.js';

function defineShape(name, NativeCtor) {
  class Shape extends Path {
    constructor(opts) {
      super();
      this._bindNative(new NativeCtor(opts ?? {}));
    }
  }
  Object.defineProperty(Shape, 'name', { value: name });
  return Shape;
}

export const Arc = defineShape('Arc', native.Arc);
export const BezierCurve = defineShape('BezierCurve', native.BezierCurve);
export const Circle = defineShape('Circle', native.Circle);
export const CompoundPath = defineShape('CompoundPath', native.CompoundPath);
export const Droplet = defineShape('Droplet', native.Droplet);
export const Ellipse = defineShape('Ellipse', native.Ellipse);
export const Heart = defineShape('Heart', native.Heart);
export const Isogon = defineShape('Isogon', native.Isogon);
export const Line = defineShape('Line', native.Line);
export const Polygon = defineShape('Polygon', native.Polygon);
export const Polyline = defineShape('Polyline', native.Polyline);
export const Rect = defineShape('Rect', native.Rect);
export const Ring = defineShape('Ring', native.Ring);
export const Rose = defineShape('Rose', native.Rose);
export const Sector = defineShape('Sector', native.Sector);
export const Star = defineShape('Star', native.Star);
export const Trochoid = defineShape('Trochoid', native.Trochoid);
