import { Displayable } from './displayable.js';
import { native } from './native.js';

export class Text extends Displayable {
  constructor(opts) {
    super();
    this.style = { fill: '#000' };
    this._bindNative(new native.Text(opts ?? {}));
    if (opts != null) {
      this.attr(opts);
    }
  }
}
