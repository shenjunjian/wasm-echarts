import { Element } from './element.js';

/**
 * 官方 Displayable。可作原型祖先；直接 `new Displayable()` 不创建 native。
 * rust `Displayable` 构造会抛错，facade 不调用它。
 */
export class Displayable extends Element {
  constructor(_opts) {
    super();
  }

  setStyle(style) {
    if (this._native && typeof this._native.setStyle === 'function') {
      this._native.setStyle(style);
    }
    return this;
  }
}
