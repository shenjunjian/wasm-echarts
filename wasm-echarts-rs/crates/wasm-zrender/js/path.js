import { Displayable } from './displayable.js';
import { native } from './native.js';

/**
 * 官方 Path。`new Path(opts)` 才创建 native；子类只 `super()`。
 */
export class Path extends Displayable {
  constructor(opts) {
    super();
    if (new.target === Path) {
      this._bindNative(new native.Path(opts ?? {}));
    }
  }

  setShape(shape) {
    if (this._native && typeof this._native.setShape === 'function') {
      this._native.setShape(shape);
    }
    return this;
  }

  setClipPath(clip) {
    if (this._native && typeof this._native.setClipPath === 'function') {
      this._native.setClipPath(clip);
    }
    return this;
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
