/**
 * 把 canvas 风格 path 命令录成 SVG pathData。
 */
export class PathRecorder {
  constructor() {
    this._d = [];
    this._x = 0;
    this._y = 0;
  }

  beginPath() {
    this._d = [];
    return this;
  }

  moveTo(x, y) {
    this._d.push('M ' + num(x) + ' ' + num(y));
    this._x = x;
    this._y = y;
    return this;
  }

  lineTo(x, y) {
    this._d.push('L ' + num(x) + ' ' + num(y));
    this._x = x;
    this._y = y;
    return this;
  }

  bezierCurveTo(x1, y1, x2, y2, x, y) {
    this._d.push(
      'C ' +
        num(x1) +
        ' ' +
        num(y1) +
        ' ' +
        num(x2) +
        ' ' +
        num(y2) +
        ' ' +
        num(x) +
        ' ' +
        num(y),
    );
    this._x = x;
    this._y = y;
    return this;
  }

  quadraticCurveTo(x1, y1, x, y) {
    this._d.push('Q ' + num(x1) + ' ' + num(y1) + ' ' + num(x) + ' ' + num(y));
    this._x = x;
    this._y = y;
    return this;
  }

  rect(x, y, w, h) {
    this.moveTo(x, y);
    this.lineTo(x + w, y);
    this.lineTo(x + w, y + h);
    this.lineTo(x, y + h);
    this.closePath();
    return this;
  }

  arc(x, y, r, startAngle, endAngle, anticlockwise) {
    const sa = startAngle == null ? 0 : startAngle;
    const ea = endAngle == null ? sa : endAngle;
    const ccw = !!anticlockwise;
    const sx = x + r * Math.cos(sa);
    const sy = y + r * Math.sin(sa);
    const ex = x + r * Math.cos(ea);
    const ey = y + r * Math.sin(ea);
    if (!this._d.length) {
      this.moveTo(sx, sy);
    } else {
      this.lineTo(sx, sy);
    }
    let delta = ea - sa;
    if (ccw) {
      if (delta > 0) {
        delta -= Math.PI * 2;
      }
    } else if (delta < 0) {
      delta += Math.PI * 2;
    }
    const large = Math.abs(delta) > Math.PI ? 1 : 0;
    const sweep = ccw ? 0 : 1;
    this._d.push(
      'A ' +
        num(r) +
        ' ' +
        num(r) +
        ' 0 ' +
        large +
        ' ' +
        sweep +
        ' ' +
        num(ex) +
        ' ' +
        num(ey),
    );
    this._x = ex;
    this._y = ey;
    return this;
  }

  closePath() {
    this._d.push('Z');
    return this;
  }

  toString() {
    return this._d.join(' ');
  }
}

function num(n) {
  const v = Number(n);
  return Number.isFinite(v) ? v : 0;
}
