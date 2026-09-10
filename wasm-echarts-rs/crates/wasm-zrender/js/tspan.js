import { Displayable } from './displayable.js';
import { native } from './native-core.js';

export class TSpan extends Displayable {
  constructor(opts) {
    super();
    this._bindNative(new native.TSpan(opts ?? {}));
    if (opts != null) {
      this.attr(opts);
    }
  }
}
