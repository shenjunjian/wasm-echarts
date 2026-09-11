/**
 * 站点 URL 前缀。本地 Vite 为 `/`；GitHub Pages 项目站为 `/wasm-echarts/`。
 */

export const SITE_BASE_URL = import.meta.env.BASE_URL || '/';

/**
 * @param {string} path 以 `/` 开头的站内路径
 */
export function withBase(path) {
  const normalized = path.startsWith('/') ? path.slice(1) : path;
  return `${SITE_BASE_URL}${normalized}`;
}

/**
 * 去掉 Pages 项目前缀，得到以 `/` 开头的站内路径。
 * @param {string} pathname
 */
export function stripBase(pathname) {
  const base = SITE_BASE_URL.replace(/\/$/, '');
  const path = pathname.replace(/\\/g, '/');
  if (base && (path === base || path.startsWith(`${base}/`))) {
    return path.slice(base.length) || '/';
  }
  return path || '/';
}

export const DEFAULT_FONT_URL = withBase('/fonts/NotoSansSC-Regular.ttf');
