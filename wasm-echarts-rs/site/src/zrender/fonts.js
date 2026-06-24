/**
 * 加载并注册 wasm-zrender 文本渲染所需字体。
 */

import { registerFont } from '@wasm-zrender/wasm_zrender.js';

const DEFAULT_FONT_URL = '/fonts/NotoSansSC-Regular.ttf';
const DEFAULT_FONT_FAMILY = 'Noto Sans SC';

/** @type {Promise<void> | null} */
let defaultFontReady = null;

/**
 * 从 URL 获取字体 bytes 并注册到 WASM fontdb。
 * @param {string} url
 * @param {{ familyName?: string, sansSerif?: string[] }} [opts]
 */
export async function loadFontFromUrl(url, opts = {}) {
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`字体加载失败: ${url} (${response.status})`);
  }
  const bytes = new Uint8Array(await response.arrayBuffer());
  const familyName = opts.familyName ?? DEFAULT_FONT_FAMILY;
  registerFont(bytes, {
    familyName,
    sansSerif: opts.sansSerif ?? [familyName],
  });
}

/** 注册默认中文字体（Noto Sans SC）。 */
export function ensureDefaultFont() {
  if (!defaultFontReady) {
    defaultFontReady = loadFontFromUrl(DEFAULT_FONT_URL, {
      familyName: DEFAULT_FONT_FAMILY,
      sansSerif: [DEFAULT_FONT_FAMILY],
    });
  }
  return defaultFontReady;
}

/**
 * 直接使用已有 font bytes 注册字体。
 * @param {Uint8Array} bytes
 * @param {{ familyName?: string, sansSerif?: string[] }} [opts]
 */
export function registerFontBytes(bytes, opts = {}) {
  const familyName = opts.familyName ?? DEFAULT_FONT_FAMILY;
  registerFont(bytes, {
    familyName,
    sansSerif: opts.sansSerif ?? [familyName],
  });
}
