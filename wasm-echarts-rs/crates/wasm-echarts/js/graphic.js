/**
 * echarts.graphic：构造器从 wasm-zrender 再导出；LinearGradient 来自同一份 echarts WASM。
 */
import { native } from './native.js';

export { Group } from '../../wasm-zrender/js/group.js';
export { Image } from '../../wasm-zrender/js/image.js';
export { Text } from '../../wasm-zrender/js/text.js';
export {
  Circle,
  Ellipse,
  Sector,
  Ring,
  Polygon,
  Polyline,
  Rect,
  Line,
  BezierCurve,
  Arc,
  CompoundPath,
} from '../../wasm-zrender/js/shapes/index.js';
export { IncrementalDisplayable } from '../../wasm-zrender/js/incremental.js';
export { BoundingRect } from '../../wasm-zrender/js/bounding_rect.js';

export const LinearGradient = native.LinearGradient;
export const RadialGradient = native.RadialGradient;
export const Pattern = native.Pattern;
