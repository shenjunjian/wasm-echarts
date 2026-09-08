function hasNativeProp(handle, name) {
  return handle != null && name in handle;
}

function toNumber(value, fallback) {
  const n = Number(value);
  return Number.isFinite(n) ? n : fallback;
}

function asPatch(keyOrObj, value) {
  if (typeof keyOrObj === 'string') {
    return { [keyOrObj]: value };
  }
  return keyOrObj != null ? keyOrObj : {};
}

function nativeHandle(el) {
  return el && el._native ? el._native : el;
}

/**
 * 官方 Element 基类。子类 `super()` 不创建 native，再 `_bindNative(handle)`。
 */
export class Element {
  constructor() {
    this._native = null;
    this._x = 0;
    this._y = 0;
    this._scaleX = 1;
    this._scaleY = 1;
    this._rotation = 0;
    this._originX = 0;
    this._originY = 0;
    this._name = '';
    this._ignore = false;
    this._silent = false;
    this.parent = null;
    this.isGroup = false;
  }

  _bindNative(handle) {
    this._native = handle;
    return this;
  }

  _syncNative(key, value) {
    if (this._native && typeof this._native.attr === 'function') {
      this._native.attr(key, value);
    }
  }

  get id() {
    return this._native ? this._native.id : undefined;
  }

  get type() {
    return this._native ? this._native.type : 'element';
  }

  get x() {
    return this._x;
  }

  set x(value) {
    this._x = toNumber(value, 0);
    this._syncNative('x', this._x);
  }

  get y() {
    return this._y;
  }

  set y(value) {
    this._y = toNumber(value, 0);
    this._syncNative('y', this._y);
  }

  get scaleX() {
    return this._scaleX;
  }

  set scaleX(value) {
    this._scaleX = toNumber(value, 1);
    this._syncNative('scaleX', this._scaleX);
  }

  get scaleY() {
    return this._scaleY;
  }

  set scaleY(value) {
    this._scaleY = toNumber(value, 1);
    this._syncNative('scaleY', this._scaleY);
  }

  get rotation() {
    return this._rotation;
  }

  set rotation(value) {
    this._rotation = toNumber(value, 0);
    this._syncNative('rotation', this._rotation);
  }

  get originX() {
    return this._originX;
  }

  set originX(value) {
    this._originX = toNumber(value, 0);
    this._syncNative('originX', this._originX);
  }

  get originY() {
    return this._originY;
  }

  set originY(value) {
    this._originY = toNumber(value, 0);
    this._syncNative('originY', this._originY);
  }

  get name() {
    return this._name;
  }

  set name(value) {
    this._name = value == null ? '' : String(value);
    this._syncNative('name', this._name);
  }

  get ignore() {
    return this._ignore;
  }

  set ignore(value) {
    this._ignore = !!value;
    this._syncNative('ignore', this._ignore);
  }

  get silent() {
    return this._silent;
  }

  set silent(value) {
    this._silent = !!value;
    this._syncNative('silent', this._silent);
  }

  get draggable() {
    return hasNativeProp(this._native, 'draggable') ? this._native.draggable : false;
  }

  set draggable(value) {
    if (hasNativeProp(this._native, 'draggable')) {
      this._native.draggable = value;
    }
  }

  get position() {
    return [this.x, this.y];
  }

  set position(value) {
    if (value && value.length >= 2) {
      this._x = toNumber(value[0], 0);
      this._y = toNumber(value[1], 0);
      this._syncNative('position', [this._x, this._y]);
    }
  }

  attrKV(key, value) {
    if (key === 'clipPath') {
      this.setClipPath(value);
    } else {
      this[key] = value;
    }
  }

  attr(keyOrObj, value) {
    if (typeof keyOrObj === 'string') {
      this.attrKV(keyOrObj, value);
    } else if (keyOrObj && typeof keyOrObj === 'object') {
      const keys = Object.keys(keyOrObj);
      for (let i = 0; i < keys.length; i++) {
        const key = keys[i];
        this.attrKV(key, keyOrObj[key]);
      }
    }
    return this;
  }

  setClipPath(clip) {
    this.clipPath = clip;
    if (this._native && typeof this._native.setClipPath === 'function') {
      this._native.setClipPath(nativeHandle(clip));
    }
    return this;
  }

  animate(path, looping) {
    if (this._native && typeof this._native.animate === 'function') {
      return this._native.animate(path, looping);
    }
    return undefined;
  }

  on(event, handler) {
    if (this._native && typeof this._native.on === 'function') {
      this._native.on(event, handler);
    }
    return this;
  }

  getBoundingRect() {
    if (this._native && typeof this._native.getBoundingRect === 'function') {
      return this._native.getBoundingRect();
    }
    return undefined;
  }
}

export { asPatch, nativeHandle, toNumber };
