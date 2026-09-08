import { Displayable } from './displayable.js';
import { native } from './native.js';

export class TSpan extends Displayable {
  constructor(opts) {
    super();
    this._bindNative(new native.TSpan(opts ?? {}));
  }
}
