function hasNativeProp(handle, name) {
  return handle != null && name in handle;
}

/**
 * 官方 Element 基类。子类 `super()` 不创建 native，再 `_bindNative(handle)`。
 */
export class Element {
  constructor() {
    this._native = null;
  }

  _bindNative(handle) {
    this._native = handle;
    return this;
  }

  get id() {
    return this._native ? this._native.id : undefined;
  }

  get type() {
    return this._native ? this._native.type : 'element';
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
    return hasNativeProp(this._native, 'position') ? this._native.position : [0, 0];
  }

  set position(value) {
    if (hasNativeProp(this._native, 'position')) {
      this._native.position = value;
    }
  }

  attr(key, value) {
    if (this._native && typeof this._native.attr === 'function') {
      this._native.attr(key, value);
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
