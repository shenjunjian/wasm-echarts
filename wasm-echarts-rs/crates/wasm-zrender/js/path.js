import { Displayable } from './displayable.js';
import { asPatch } from './element.js';
import { native } from './native-core.js';
import { PathRecorder } from './path_recorder.js';

export { PathRecorder };

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

  setStateStyle(state, style) {
    if (this._native && typeof this._native.setStateStyle === 'function') {
      this._native.setStateStyle(state, style);
    }
    return this;
  }
}

function clonePlain(obj) {
  if (!obj || typeof obj !== 'object') {
    return {};
  }
  return { ...obj };
}

Path.extend = function (defaultProps) {
  const props = defaultProps || {};
  class Sub extends Path {
    constructor(opts) {
      super();
      const shape = {
        ...clonePlain(typeof this.getDefaultShape === 'function' ? this.getDefaultShape() : props.shape),
        ...(opts && opts.shape),
      };
      const style = {
        ...clonePlain(typeof this.getDefaultStyle === 'function' ? this.getDefaultStyle() : props.style),
        ...(opts && opts.style),
      };
      this.shape = shape;
      this.style = { ...(this.style || {}), ...style };
      if (typeof this.buildPath === 'function') {
        const recorder = new PathRecorder();
        this.buildPath(recorder, this.shape);
        const pathData = recorder.toString();
        if (pathData) {
          this.shape.pathData = pathData;
        }
      }
      this._bindNative(
        new native.Path({
          ...(opts || {}),
          shape: this.shape,
          style: this.style,
        }),
      );
      if (opts != null) {
        this.attr(opts);
      }
      if (typeof props.init === 'function') {
        props.init.call(this, opts);
      }
    }

    getDefaultShape() {
      return clonePlain(props.shape);
    }

    getDefaultStyle() {
      return clonePlain(props.style);
    }
  }

  const keys = Object.keys(props);
  for (let i = 0; i < keys.length; i++) {
    const key = keys[i];
    if (typeof props[key] === 'function') {
      Sub.prototype[key] = props[key];
    }
  }
  if (props.type) {
    Object.defineProperty(Sub.prototype, 'type', {
      get() {
        return props.type;
      },
    });
  }
  return Sub;
};
