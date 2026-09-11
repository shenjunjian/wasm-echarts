/**
 * 全站顶栏：品牌回首页，产品入口进文档默认页；进入产品后旁挂「文档 / 实例」。
 */

const PRODUCTS = [
  {
    id: 'zrender',
    label: 'wasm-zrender',
    href: '/zrender/docs/',
    docs: '/zrender/docs/',
    examples: '/zrender/examples/',
  },
  {
    id: 'echarts',
    label: 'wasm-echarts',
    href: '/echarts/docs/',
    docs: '/echarts/docs/',
    examples: '/echarts/examples/',
  },
];

/**
 * @param {string} pathname
 */
function normalizePath(pathname) {
  const path = pathname.replace(/\\/g, '/').replace(/\/index\.html$/i, '');
  if (path.length > 1 && path.endsWith('/')) return path.slice(0, -1);
  return path || '/';
}

/**
 * @param {string} pathname
 * @returns {{ product: 'zrender' | 'echarts' | null, section: 'docs' | 'examples' | null }}
 */
export function detectSiteContext(pathname = location.pathname) {
  const path = normalizePath(pathname);
  let product = null;
  if (path === '/zrender' || path.startsWith('/zrender/')) product = 'zrender';
  else if (path === '/echarts' || path.startsWith('/echarts/')) product = 'echarts';

  let section = null;
  if (product) {
    if (path.includes('/docs')) section = 'docs';
    else if (path.includes('/examples')) section = 'examples';
  }
  return { product, section };
}

/**
 * @param {HTMLElement} [root]
 * @param {{ product?: string | null, section?: string | null }} [opts]
 */
export function mountSiteHeader(root = document.querySelector('header.site-header'), opts = {}) {
  if (!root) return;

  const detected = detectSiteContext();
  const product = opts.product !== undefined ? opts.product : (root.dataset.product || detected.product);
  const section = opts.section !== undefined ? opts.section : (root.dataset.section || detected.section);

  const clusters = PRODUCTS.map((item) => {
    const isCurrent = item.id === product;
    const sub = isCurrent
      ? `<span class="site-nav-sub">
          <a href="${item.docs}"${section === 'docs' ? ' class="is-active" aria-current="page"' : ''}>文档</a>
          <a href="${item.examples}"${section === 'examples' ? ' class="is-active" aria-current="page"' : ''}>实例</a>
        </span>`
      : '';
    return `<div class="site-nav-item${isCurrent ? ' is-current' : ''}">
      <a class="site-nav-product" href="${item.href}">${item.label}</a>
      ${sub}
    </div>`;
  }).join('');

  root.innerHTML = `
    <a class="brand" href="/">wasm-echarts</a>
    <nav class="site-nav" aria-label="产品">${clusters}</nav>
  `;
}

if (typeof document !== 'undefined') {
  const header = document.querySelector('header.site-header');
  if (header) mountSiteHeader(header);
}
