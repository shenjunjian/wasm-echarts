/** 官方 Point 静态方法（对 PointLike `{x,y}` 操作）。 */
export function attachPointStatics(Point) {
  Point.set = function (p, x, y) {
    p.x = x;
    p.y = y;
  };
  Point.copy = function (p, p2) {
    p.x = p2.x;
    p.y = p2.y;
  };
  Point.len = function (p) {
    return Math.sqrt(p.x * p.x + p.y * p.y);
  };
  Point.lenSquare = function (p) {
    return p.x * p.x + p.y * p.y;
  };
  Point.dot = function (p0, p1) {
    return p0.x * p1.x + p0.y * p1.y;
  };
  Point.add = function (out, p0, p1) {
    out.x = p0.x + p1.x;
    out.y = p0.y + p1.y;
  };
  Point.sub = function (out, p0, p1) {
    out.x = p0.x - p1.x;
    out.y = p0.y - p1.y;
  };
  Point.scale = function (out, p0, scalar) {
    out.x = p0.x * scalar;
    out.y = p0.y * scalar;
  };
  Point.scaleAndAdd = function (out, p0, p1, scalar) {
    out.x = p0.x + p1.x * scalar;
    out.y = p0.y + p1.y * scalar;
  };
  Point.lerp = function (out, p0, p1, t) {
    const onet = 1 - t;
    out.x = onet * p0.x + t * p1.x;
    out.y = onet * p0.y + t * p1.y;
  };
  return Point;
}
