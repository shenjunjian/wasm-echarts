/**
 * 官方 setPlatformAPI。离屏 WASM 不读系统字体；这里只保留可替换的宿主钩子。
 */

const DEFAULT_FONT_SIZE = 12;
const DEFAULT_FONT = `${DEFAULT_FONT_SIZE}px sans-serif`;

export const platformApi = {
  createCanvas() {
    if (typeof document !== 'undefined' && document.createElement) {
      return document.createElement('canvas');
    }
    return null;
  },

  measureText(text, font) {
    text = text || '';
    font = font || DEFAULT_FONT;
    if (typeof document !== 'undefined') {
      const canvas = platformApi.createCanvas();
      const ctx = canvas && canvas.getContext && canvas.getContext('2d');
      if (ctx) {
        ctx.font = font;
        return ctx.measureText(text);
      }
    }
    const match = /((?:\d+)?\.?\d*)px/.exec(font);
    const fontSize = (match && +match[1]) || DEFAULT_FONT_SIZE;
    const mono = font.indexOf('mono') >= 0;
    return { width: mono ? fontSize * text.length : fontSize * text.length * 0.6 };
  },

  loadImage(src, onload, onerror) {
    if (typeof Image === 'undefined') {
      return null;
    }
    const image = new Image();
    image.onload = onload;
    image.onerror = onerror;
    image.src = src;
    return image;
  },

  getTime() {
    return Date.now ? Date.now() : +new Date();
  },
};

export function setPlatformAPI(apis) {
  if (!apis) {
    return;
  }
  for (const key of Object.keys(platformApi)) {
    if (Object.prototype.hasOwnProperty.call(apis, key) && apis[key]) {
      platformApi[key] = apis[key];
    }
  }
}
