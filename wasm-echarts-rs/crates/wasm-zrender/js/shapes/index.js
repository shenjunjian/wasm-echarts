import { Path } from '../path.js';
import { native } from '../native.js';

const STROKE_DEFAULT_STYLE = { stroke: '#000', fill: null };

function defineShape(name, NativeCtor, defaultStyle) {
  class Shape extends Path {
    constructor(opts) {
      super();
      if (defaultStyle) {
        this.style = { ...defaultStyle };
      }
      this._bindNative(new NativeCtor(opts ?? {}));
      if (opts != null) {
        this.attr(opts);
      }
    }
  }
  Object.defineProperty(Shape, 'name', { value: name });
  return Shape;
}

export const Arc = defineShape('Arc', native.Arc, STROKE_DEFAULT_STYLE);
export const BezierCurve = defineShape('BezierCurve', native.BezierCurve, STROKE_DEFAULT_STYLE);
export const Circle = defineShape('Circle', native.Circle);
export const CompoundPath = defineShape('CompoundPath', native.CompoundPath);
export const Droplet = defineShape('Droplet', native.Droplet);
export const Ellipse = defineShape('Ellipse', native.Ellipse);
export const Heart = defineShape('Heart', native.Heart);
export const Isogon = defineShape('Isogon', native.Isogon);
export const Line = defineShape('Line', native.Line, STROKE_DEFAULT_STYLE);
export const Polygon = defineShape('Polygon', native.Polygon);
export const Polyline = defineShape('Polyline', native.Polyline, STROKE_DEFAULT_STYLE);
export const Rect = defineShape('Rect', native.Rect);
export const Ring = defineShape('Ring', native.Ring);
export const Rose = defineShape('Rose', native.Rose, STROKE_DEFAULT_STYLE);
export const Sector = defineShape('Sector', native.Sector);
export const Star = defineShape('Star', native.Star);
export const Trochoid = defineShape('Trochoid', native.Trochoid, STROKE_DEFAULT_STYLE);
