/**
 * wasm-zrender 文档壳：左侧目录、当前页高亮、移动端折叠。
 * 顶栏复用 site-header（同一套产品菜单）。
 */

import { mountSiteHeader } from './site-header.js';

const DOCS_BASE = '/zrender/docs';

/** @typedef {{ label: string, href?: string, id?: string, children?: DocNavItem[] }} DocNavItem */

/** @type {DocNavItem[]} */
export const DOC_NAV = [
  { id: 'start', label: '快速上手', href: `${DOCS_BASE}/` },
  { id: 'fonts', label: '字体引用', href: `${DOCS_BASE}/fonts.html` },
  {
    id: 'api',
    label: 'API 参考',
    href: `${DOCS_BASE}/api/`,
    children: [
      {
        id: 'api-facade',
        label: 'JS facade API',
        href: `${DOCS_BASE}/api/facade/`,
        children: [
          { id: 'facade-overview', label: '总览与约定', href: `${DOCS_BASE}/api/facade/` },
          { id: 'facade-lifecycle', label: '生命周期', href: `${DOCS_BASE}/api/facade/lifecycle.html` },
          { id: 'facade-instance', label: 'ZRender 实例', href: `${DOCS_BASE}/api/facade/instance.html` },
          { id: 'facade-element', label: 'Element / Displayable / Path / Group', href: `${DOCS_BASE}/api/facade/element.html` },
          { id: 'facade-shapes', label: '图元（Shape / Text / Image）', href: `${DOCS_BASE}/api/facade/shapes.html` },
          { id: 'facade-style', label: '样式与几何', href: `${DOCS_BASE}/api/facade/style.html` },
          { id: 'facade-tools', label: '工具模块', href: `${DOCS_BASE}/api/facade/tools.html` },
          { id: 'facade-animation', label: '动画与事件', href: `${DOCS_BASE}/api/facade/animation.html` },
        ],
      },
      {
        id: 'api-pkg',
        label: 'pkg 导出 API',
        href: `${DOCS_BASE}/api/pkg/`,
        children: [
          { id: 'pkg-overview', label: '总览与何时使用', href: `${DOCS_BASE}/api/pkg/` },
          { id: 'pkg-zrender', label: '顶层函数与 ZRender', href: `${DOCS_BASE}/api/pkg/zrender.html` },
          { id: 'pkg-graphic', label: '图元与样式类', href: `${DOCS_BASE}/api/pkg/graphic.html` },
          { id: 'pkg-unused', label: '空壳与不要直接用的符号', href: `${DOCS_BASE}/api/pkg/unused.html` },
        ],
      },
    ],
  },
  { id: 'diff', label: '与官方差异', href: `${DOCS_BASE}/differences.html` },
  { id: 'internals', label: '底层原理', href: `${DOCS_BASE}/internals.html` },
];

/** @type {Record<string, { className: string, label: string }>} */
export const DOC_BADGES = {
  official: { className: 'doc-badge-official', label: '官方' },
  extra: { className: 'doc-badge-extra', label: '补充' },
  degraded: { className: 'doc-badge-degraded', label: '降级' },
  facade: { className: 'doc-badge-facade', label: '仅 facade' },
  pkg: { className: 'doc-badge-pkg', label: '仅 pkg' },
};

/**
 * @param {keyof typeof DOC_BADGES} kind
 */
export function badgeHtml(kind) {
  const item = DOC_BADGES[kind];
  if (!item) return '';
  return `<span class="doc-badge ${item.className}">${escapeHtml(item.label)}</span>`;
}

/**
 * @param {string} pathname
 */
function normalizePath(pathname) {
  const path = pathname.replace(/\\/g, '/').replace(/\/index\.html$/i, '');
  if (path.length > 1 && path.endsWith('/')) return path.slice(0, -1);
  return path || '/';
}

/**
 * @param {string} href
 * @param {string} currentPath
 */
function isCurrentHref(href, currentPath) {
  return normalizePath(href) === currentPath;
}

/**
 * 子页路径落在该 href 之下（不含自身）。用于展开祖先，避免 `/docs` 把所有页都当成当前。
 * @param {string} href
 * @param {string} currentPath
 */
function isAncestorHref(href, currentPath) {
  const base = normalizePath(href);
  return currentPath.startsWith(`${base}/`);
}

/**
 * @param {DocNavItem} item
 * @param {string} currentPath
 */
function itemCoversPath(item, currentPath) {
  if (item.href && (isCurrentHref(item.href, currentPath) || isAncestorHref(item.href, currentPath))) {
    return true;
  }
  return Boolean(item.children?.some((child) => itemCoversPath(child, currentPath)));
}

/**
 * 当前页高亮：叶子按 href 精确匹配；分组与子页同 href 时只亮叶子。
 * @param {DocNavItem} item
 * @param {string} currentPath
 */
function isActiveItem(item, currentPath) {
  if (!item.href || !isCurrentHref(item.href, currentPath)) return false;
  if (!item.children?.length) return true;
  const childSamePage = item.children.some((child) => child.href && isCurrentHref(child.href, currentPath));
  if (childSamePage) return false;
  return !item.children.some((child) => itemCoversPath(child, currentPath));
}

function collectOpenIds(items, currentPath, acc = new Set()) {
  for (const item of items) {
    if (item.children?.length && itemCoversPath(item, currentPath)) {
      acc.add(item.id);
      collectOpenIds(item.children, currentPath, acc);
    }
  }
  return acc;
}

function escapeHtml(value) {
  return String(value)
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;');
}

function escapeAttr(value) {
  return escapeHtml(value).replace(/'/g, '&#39;');
}

/**
 * @param {DocNavItem} item
 * @param {string} currentPath
 * @param {Set<string>} openIds
 * @param {number} depth
 */
function renderItem(item, currentPath, openIds, depth) {
  const hasChildren = Boolean(item.children?.length);
  const active = isActiveItem(item, currentPath);
  const linkClass = `docs-nav-link${active ? ' is-active' : ''}${hasChildren ? ' is-group' : ''}`;
  const currentAttr = active ? ' aria-current="page"' : '';
  const label = item.href
    ? `<a class="${linkClass}" href="${escapeAttr(item.href)}"${currentAttr}>${escapeHtml(item.label)}</a>`
    : `<span class="${linkClass}">${escapeHtml(item.label)}</span>`;

  if (!hasChildren) {
    return `<div class="docs-nav-row" data-depth="${depth}">${label}</div>`;
  }

  const open = openIds.has(item.id);
  const covers = itemCoversPath(item, currentPath);
  return `<div class="docs-nav-group${open ? ' is-open' : ''}${covers ? ' has-active' : ''}" data-group="${escapeAttr(item.id)}">
    <div class="docs-nav-row docs-nav-group-head" data-depth="${depth}">
      <button type="button" class="docs-nav-group-toggle" data-group="${escapeAttr(item.id)}" aria-expanded="${open ? 'true' : 'false'}" aria-label="${escapeAttr(`展开或折叠 ${item.label}`)}"></button>
      ${label}
    </div>
    <div class="docs-nav-children">${item.children.map((child) => renderItem(child, currentPath, openIds, depth + 1)).join('')}</div>
  </div>`;
}

function renderLegend() {
  return `<div class="docs-legend" aria-label="API 徽章说明">
    <p class="docs-legend-title">徽章</p>
    <ul>
      <li>${badgeHtml('official')} 对齐官方 export</li>
      <li>${badgeHtml('extra')} wasm-zrender 多出来的</li>
      <li>${badgeHtml('degraded')} 签名在、语义弱于官方</li>
      <li>${badgeHtml('facade')} 只在 JS facade</li>
      <li>${badgeHtml('pkg')} 只在 pkg handle</li>
    </ul>
  </div>`;
}

/**
 * @param {HTMLElement} [root]
 * @param {{ nav?: DocNavItem[], pathname?: string }} [opts]
 */
export function mountDocsShell(root = document.querySelector('.docs-sidebar'), opts = {}) {
  if (!root) return;

  const nav = opts.nav || DOC_NAV;
  const currentPath = normalizePath(opts.pathname || location.pathname);
  const openIds = collectOpenIds(nav, currentPath);

  const render = () => {
    const open = root.classList.contains('is-open');
    root.innerHTML = `
      <div class="docs-sidebar-header">
        <p class="docs-sidebar-kicker">wasm-zrender</p>
        <div class="docs-sidebar-title-row">
          <h1>文档</h1>
          <button type="button" class="docs-menu-toggle" aria-expanded="${open ? 'true' : 'false'}" aria-controls="docs-nav">
            ${open ? '收起' : '目录'}
          </button>
        </div>
      </div>
      <nav id="docs-nav" class="docs-nav" aria-label="文档目录">
        ${nav.map((item) => renderItem(item, currentPath, openIds, 0)).join('')}
      </nav>
      ${renderLegend()}
    `;
  };

  render();

  root.addEventListener('click', (event) => {
    const toggleMenu = event.target.closest('.docs-menu-toggle');
    if (toggleMenu) {
      root.classList.toggle('is-open');
      render();
      return;
    }

    const groupBtn = event.target.closest('[data-group]');
    if (groupBtn && groupBtn.matches('button')) {
      event.preventDefault();
      const id = groupBtn.dataset.group;
      if (openIds.has(id)) openIds.delete(id);
      else openIds.add(id);
      render();
    }
  });
}

/**
 * 供核对该路径会高亮哪一项、展开哪些分组。
 * @param {string} pathname
 */
export function inspectDocsNav(pathname) {
  const currentPath = normalizePath(pathname);
  const openIds = [...collectOpenIds(DOC_NAV, currentPath)];
  /** @type {string[]} */
  const actives = [];
  /** @param {DocNavItem[]} items */
  const walk = (items) => {
    for (const item of items) {
      if (isActiveItem(item, currentPath)) actives.push(item.id || item.label);
      if (item.children) walk(item.children);
    }
  };
  walk(DOC_NAV);
  return { currentPath, openIds, actives };
}

if (typeof document !== 'undefined') {
  mountSiteHeader();
  const sidebar = document.querySelector('.docs-sidebar');
  if (sidebar) mountDocsShell(sidebar);
}
