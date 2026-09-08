import { Displayable } from './displayable.js';
import { native } from './native.js';

/**
 * 波次 6 再按普通 Group 语义落地。当前委托 rust，构造仍会抛错。
 */
export class IncrementalDisplayable extends Displayable {
  constructor(opts) {
    super();
    this._bindNative(new native.IncrementalDisplayable(opts));
  }
}
