import { Displayable } from './displayable.js';
import { native } from './native.js';

export class Text extends Displayable {
  constructor(opts) {
    super();
    this._bindNative(new native.Text(opts ?? {}));
    if (opts != null) {
      this.attr(opts);
    }
  }
}
