import { highlightSource } from './highlight-source.js';

/**
 * 实例画廊：左侧菜单 · 右侧源码 + iframe 预览
 */

/**
 * @typedef {{
 *   id: string;
 *   title: string;
 *   description?: string;
 *   previewUrl: string;
 *   source: string;
 * }} GalleryExample
 *
 * @typedef {{
 *   id: string;
 *   title: string;
 *   examples: GalleryExample[];
 * }} GalleryGroup
 *
 * @param {HTMLElement} root
 * @param {{
 *   title: string;
 *   description?: string;
 *   examples?: GalleryExample[];
 *   groups?: GalleryGroup[];
 *   defaultId?: string;
 * }} options
 */
export function mountExampleGallery(root, options) {
  const groups = options.groups?.length ? options.groups : null;
  const examples = groups
    ? groups.flatMap((group) => {
        if (!group.examples?.length) {
          throw new Error(`分组「${group.title || group.id}」的 examples 不能为空`);
        }
        return group.examples;
      })
    : options.examples;
  if (!examples?.length) {
    throw new Error('examples 不能为空');
  }

  const ids = new Set();
  for (const item of examples) {
    if (ids.has(item.id)) {
      throw new Error(`重复的示例 id: ${item.id}`);
    }
    ids.add(item.id);
  }

  const exampleGroupId = new Map();
  if (groups) {
    for (const group of groups) {
      for (const item of group.examples) {
        exampleGroupId.set(item.id, group.id);
      }
    }
  }

  const pickInitial = () => {
    const hash = location.hash.replace(/^#/, '');
    if (hash && ids.has(hash)) return hash;
    if (options.defaultId && ids.has(options.defaultId)) return options.defaultId;
    return examples[0].id;
  };

  /** @type {Set<string>} */
  const expandedGroupIds = new Set(groups ? groups.map((group) => group.id) : []);

  root.className = 'example-gallery';
  root.innerHTML = `
    <aside class="example-sidebar">
      <div class="example-sidebar-header">
        <h1>${escapeHtml(options.title)}</h1>
        ${options.description ? `<p>${escapeHtml(options.description)}</p>` : ''}
      </div>
      <nav class="example-nav" aria-label="实例菜单"></nav>
    </aside>
    <div class="example-main">
      <section class="example-source-panel">
        <div class="example-panel-header">
          <span>源码</span>
          <button type="button" class="btn btn-sm" data-action="copy">复制</button>
        </div>
        <div class="example-source-code"></div>
      </section>
      <section class="example-preview-panel">
        <div class="example-panel-header">预览</div>
        <iframe title="实例预览" loading="lazy"></iframe>
      </section>
    </div>
  `;

  const nav = root.querySelector('.example-nav');
  const sourceEl = root.querySelector('.example-source-code');
  const iframe = root.querySelector('iframe');
  const copyBtn = root.querySelector('[data-action="copy"]');

  /** @type {string} */
  let activeId = pickInitial();

  const getExample = (id) => examples.find((item) => item.id === id) ?? examples[0];

  const renderLeaf = (item) => `
      <button
        type="button"
        class="example-nav-item${item.id === activeId ? ' is-active' : ''}"
        data-id="${escapeAttr(item.id)}"
      >
        <span class="example-nav-title">${escapeHtml(item.title)}</span>
        ${item.description ? `<span class="example-nav-desc">${escapeHtml(item.description)}</span>` : ''}
      </button>
    `;

  const renderNav = () => {
    if (!groups) {
      nav.innerHTML = examples.map(renderLeaf).join('');
      return;
    }

    nav.innerHTML = groups
      .map((group) => {
        const expanded = expandedGroupIds.has(group.id);
        const hasActive = group.examples.some((item) => item.id === activeId);
        return `
      <div class="example-nav-group${expanded ? ' is-open' : ''}${hasActive ? ' has-active' : ''}">
        <button
          type="button"
          class="example-nav-group-toggle"
          data-group="${escapeAttr(group.id)}"
          aria-expanded="${expanded ? 'true' : 'false'}"
        >
          <span class="example-nav-group-label">${escapeHtml(group.title)}</span>
          <span class="example-nav-group-count">${group.examples.length}</span>
        </button>
        ${expanded ? `<div class="example-nav-children">${group.examples.map(renderLeaf).join('')}</div>` : ''}
      </div>
    `;
      })
      .join('');
  };

  /** @type {number} */
  let renderSeq = 0;

  const renderSource = async (source) => {
    const seq = ++renderSeq;
    sourceEl.textContent = source;
    try {
      const html = await highlightSource(source);
      if (seq !== renderSeq) return;
      sourceEl.innerHTML = html;
    } catch (err) {
      if (seq !== renderSeq) return;
      console.error(err);
      sourceEl.textContent = source;
    }
  };

  const selectExample = (id, { pushHash = true } = {}) => {
    if (!ids.has(id)) return;
    activeId = id;
    const example = getExample(id);
    const groupId = exampleGroupId.get(id);
    if (groupId) expandedGroupIds.add(groupId);

    renderNav();
    void renderSource(example.source);
    iframe.src = example.previewUrl;

    if (pushHash && location.hash !== `#${id}`) {
      history.replaceState(null, '', `#${id}`);
    }
  };

  nav.addEventListener('click', (event) => {
    const groupBtn = event.target.closest('[data-group]');
    if (groupBtn) {
      const groupId = groupBtn.dataset.group;
      if (expandedGroupIds.has(groupId)) {
        expandedGroupIds.delete(groupId);
      } else {
        expandedGroupIds.add(groupId);
      }
      renderNav();
      return;
    }
    const btn = event.target.closest('[data-id]');
    if (!btn) return;
    selectExample(btn.dataset.id);
  });

  copyBtn.addEventListener('click', async () => {
    const text = getExample(activeId).source;
    try {
      await navigator.clipboard.writeText(text);
      copyBtn.textContent = '已复制';
      setTimeout(() => {
        copyBtn.textContent = '复制';
      }, 1500);
    } catch {
      copyBtn.textContent = '复制失败';
      setTimeout(() => {
        copyBtn.textContent = '复制';
      }, 1500);
    }
  });

  window.addEventListener('hashchange', () => {
    const hash = location.hash.replace(/^#/, '');
    if (hash && ids.has(hash) && hash !== activeId) {
      selectExample(hash, { pushHash: false });
    }
  });

  selectExample(activeId, { pushHash: false });
  if (location.hash !== `#${activeId}`) {
    history.replaceState(null, '', `#${activeId}`);
  }
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
