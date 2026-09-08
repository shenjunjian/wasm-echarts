import { Element } from './element.js';
import { native } from './native.js';

/**
 * 官方 Group extends Element（不是 Displayable）。
 * opts 解析（x/y 等）留到波次 2；现有 attr 能处理的字段先委托 native。
 */
export class Group extends Element {
  constructor(opts) {
    super();
    this._bindNative(new native.Group());
    if (opts != null) {
      this.attr(opts);
    }
  }

  add(child) {
    if (this._native) {
      this._native.add(child);
    }
    return child;
  }

  remove(child) {
    if (this._native) {
      this._native.remove(child);
    }
    return child;
  }

  removeAll() {
    if (this._native && typeof this._native.removeAll === 'function') {
      this._native.removeAll();
    }
    return this;
  }
}
