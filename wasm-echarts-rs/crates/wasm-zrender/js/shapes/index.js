import { Path } from '../path.js';
import { native } from '../native-core.js';

const STROKE_DEFAULT_STYLE = { stroke: '#000', fill: null };

function defineShape(name, NativeCtor, defaultStyle) {
  class Shape extends Path {
    constructor(opts) {
      super();
      if (defaultStyle) {
        this.style = { ...defaultStyle };
      }
      const Ctor = NativeCtor || native[name];
      this._bindNative(new Ctor(opts ?? {}));
      if (opts != null) {
        this.attr(opts);
      }
    }
  }
  Object.defineProperty(Shape, 'name', { value: name });
  return Shape;
}

export const Arc = defineShape('Arc', null, STROKE_DEFAULT_STYLE);
export const BezierCurve = defineShape('BezierCurve', null, STROKE_DEFAULT_STYLE);
export const Circle = defineShape('Circle');
export const CompoundPath = defineShape('CompoundPath');
export const Droplet = defineShape('Droplet');
export const Ellipse = defineShape('Ellipse');
export const Heart = defineShape('Heart');
export const Isogon = defineShape('Isogon');
export const Line = defineShape('Line', null, STROKE_DEFAULT_STYLE);
export const Polygon = defineShape('Polygon');
export const Polyline = defineShape('Polyline', null, STROKE_DEFAULT_STYLE);
export const Rect = defineShape('Rect');
export const Ring = defineShape('Ring');
export const Rose = defineShape('Rose', null, STROKE_DEFAULT_STYLE);
export const Sector = defineShape('Sector');
export const Star = defineShape('Star');
export const Trochoid = defineShape('Trochoid', null, STROKE_DEFAULT_STYLE);
