/**
 * 全站顶栏：品牌回首页；产品名为下拉，内挂「文档 / 实例」。
 */

const PRODUCTS = [
  {
    id: 'zrender',
    label: 'wasm-zrender',
    docs: '/zrender/docs/',
    examples: '/zrender/examples/',
  },
  {
    id: 'echarts',
    label: 'wasm-echarts',
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

function prefersFineHover() {
  return window.matchMedia('(hover: hover) and (pointer: fine)').matches;
}

/**
 * @param {HTMLElement} root
 * @param {HTMLElement | null} except
 */
function setOpenItem(root, except) {
  root.querySelectorAll('.site-nav-item').forEach((item) => {
    const open = item === except;
    item.classList.toggle('is-open', open);
    item.querySelector('.site-nav-product')?.setAttribute('aria-expanded', open ? 'true' : 'false');
  });
}

function closeAllNavs() {
  document.querySelectorAll('header.site-header').forEach((header) => setOpenItem(header, null));
}

/**
 * @param {HTMLElement} root
 */
function bindNavEvents(root) {
  if (root.dataset.navBound === '1') return;
  root.dataset.navBound = '1';

  root.addEventListener('click', (event) => {
    const btn = event.target.closest('.site-nav-product');
    if (!btn || !root.contains(btn)) return;
    if (prefersFineHover()) return;
    const item = btn.closest('.site-nav-item');
    const willOpen = !item.classList.contains('is-open');
    setOpenItem(root, willOpen ? item : null);
  });

  root.addEventListener('pointerover', (event) => {
    if (!prefersFineHover()) return;
    const item = event.target.closest('.site-nav-item');
    if (item && root.contains(item)) setOpenItem(root, item);
  });

  root.addEventListener('pointerout', (event) => {
    if (!prefersFineHover()) return;
    const item = event.target.closest('.site-nav-item');
    const related = event.relatedTarget instanceof Element
      ? event.relatedTarget.closest('.site-nav-item')
      : null;
    if (item && related !== item) setOpenItem(root, null);
  });

  if (document.documentElement.dataset.siteNavDocBound === '1') return;
  document.documentElement.dataset.siteNavDocBound = '1';

  document.addEventListener('click', (event) => {
    if (event.target.closest('.site-nav-item')) return;
    closeAllNavs();
  });

  document.addEventListener('keydown', (event) => {
    if (event.key !== 'Escape') return;
    const openBtn = document.querySelector('.site-nav-item.is-open .site-nav-product');
    closeAllNavs();
    openBtn?.focus();
  });
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
    const docsActive = isCurrent && section === 'docs';
    const examplesActive = isCurrent && section === 'examples';
    const menuId = `site-nav-menu-${item.id}`;
    return `<div class="site-nav-item${isCurrent ? ' is-current' : ''}">
      <button type="button" class="site-nav-product" aria-expanded="false" aria-haspopup="true" aria-controls="${menuId}">
        ${item.label}
      </button>
      <div class="site-nav-menu" id="${menuId}">
        <a href="${item.docs}"${docsActive ? ' class="is-active" aria-current="page"' : ''}>文档</a>
        <a href="${item.examples}"${examplesActive ? ' class="is-active" aria-current="page"' : ''}>实例</a>
      </div>
    </div>`;
  }).join('');

  root.innerHTML = `
    <a class="brand" href="/">wasm-echarts</a>
    <nav class="site-nav" aria-label="产品">${clusters}</nav>
  `;
  bindNavEvents(root);
}

if (typeof document !== 'undefined') {
  const header = document.querySelector('header.site-header');
  if (header) mountSiteHeader(header);
}
