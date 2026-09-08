import { Displayable } from './displayable.js';
import { native } from './native.js';

export class Image extends Displayable {
  constructor(opts) {
    super();
    this._bindNative(new native.Image(opts ?? {}));
  }
}
