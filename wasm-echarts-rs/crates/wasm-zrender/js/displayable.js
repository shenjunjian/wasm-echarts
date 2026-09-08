import { Element, asPatch } from './element.js';

/**
 * 官方 Displayable。可作原型祖先；直接 `new Displayable()` 不创建 native。
 * rust `Displayable` 构造会抛错，facade 不调用它。
 */
export class Displayable extends Element {
  constructor(_opts) {
    super();
    this._z = 0;
    this._z2 = 0;
    this._zlevel = 0;
    this._invisible = false;
    this.style = this.style || {};
  }

  get z() {
    return this._z;
  }

  set z(value) {
    this._z = Number(value) || 0;
    this._syncNative('z', this._z);
  }

  get z2() {
    return this._z2;
  }

  set z2(value) {
    this._z2 = Number(value) || 0;
    this._syncNative('z2', this._z2);
  }

  get zlevel() {
    return this._zlevel;
  }

  set zlevel(value) {
    this._zlevel = Number(value) || 0;
    this._syncNative('zlevel', this._zlevel);
  }

  get invisible() {
    return this._invisible;
  }

  set invisible(value) {
    this._invisible = !!value;
    this._syncNative('invisible', this._invisible);
  }

  attrKV(key, value) {
    if (key === 'style') {
      this.setStyle(value);
    } else {
      super.attrKV(key, value);
    }
  }

  setStyle(keyOrObj, value) {
    const patch = asPatch(keyOrObj, value);
    if (!this.style) {
      this.style = {};
    }
    Object.assign(this.style, patch);
    if (this._native && typeof this._native.setStyle === 'function') {
      this._native.setStyle(patch);
    }
    return this;
  }
}
