import { lazyNativeClass } from './native-core.js';
import { attachPointStatics } from './point_statics.js';

export const Point = lazyNativeClass('Point', attachPointStatics);
