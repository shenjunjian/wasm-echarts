import { Displayable } from './displayable.js';
import { asPatch } from './element.js';
import { native } from './native.js';

/**
 * 官方 Path。`new Path(opts)` 才创建 native；子类只 `super()`。
 */
export class Path extends Displayable {
  constructor(opts) {
    super();
    this.shape = this.shape || {};
    if (new.target === Path) {
      this._bindNative(new native.Path(opts ?? {}));
      if (opts != null) {
        this.attr(opts);
      }
    }
  }

  attrKV(key, value) {
    if (key === 'shape') {
      this.setShape(value);
    } else {
      super.attrKV(key, value);
    }
  }

  setShape(keyOrObj, value) {
    const patch = asPatch(keyOrObj, value);
    if (!this.shape) {
      this.shape = {};
    }
    Object.assign(this.shape, patch);
    if (this._native && typeof this._native.setShape === 'function') {
      this._native.setShape(patch);
    }
    return this;
  }

  setClipPath(clip) {
    return super.setClipPath(clip);
  }

  useState(state) {
    if (this._native && typeof this._native.useState === 'function') {
      this._native.useState(state);
    }
    return this;
  }

  setStateStyle(state, style) {
    if (this._native && typeof this._native.setStateStyle === 'function') {
      this._native.setStateStyle(state, style);
    }
    return this;
  }
}
