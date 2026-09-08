/**
 * wasm-zrender 公开 API：官方 export.ts / zrender.ts 命名空间。
 * default 仍是 wasm-bindgen 的 initWasm。
 */
export { default } from './native.js';

export {
  init,
  dispose,
  disposeAll,
  getInstance,
  registerPainter,
  version,
  ZRender,
} from './zrender.js';

export { Element } from './element.js';
export { Displayable } from './displayable.js';
export { Path } from './path.js';
export { Group } from './group.js';
export { Text } from './text.js';
export { Image } from './image.js';
export { TSpan } from './tspan.js';
export { IncrementalDisplayable } from './incremental.js';

export {
  Arc,
  BezierCurve,
  Circle,
  CompoundPath,
  Droplet,
  Ellipse,
  Heart,
  Isogon,
  Line,
  Polygon,
  Polyline,
  Rect,
  Ring,
  Rose,
  Sector,
  Star,
  Trochoid,
} from './shapes/index.js';

export {
  registerFont,
  clearFonts,
  LinearGradient,
  RadialGradient,
  Pattern,
  Point,
  BoundingRect,
  OrientedBoundingRect,
  Animation,
  Animator,
  Handler,
  HoverResult,
  matrix,
  vector,
  color,
  path,
  util,
  morph,
  parseSVG,
  showDebugDirtyRect,
  setPlatformAPI,
} from './native.js';
