import { Displayable } from './displayable.js';
import { native } from './native-core.js';

export class Image extends Displayable {
  constructor(opts) {
    super();
    this._bindNative(new native.Image(opts ?? {}));
    if (opts != null) {
      this.attr(opts);
    }
  }
}
