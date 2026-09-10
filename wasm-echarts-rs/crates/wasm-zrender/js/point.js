import { native } from './native-core.js';
import { attachPointStatics } from './point_statics.js';

const Point = native.Point;
attachPointStatics(Point);

export { Point };
