/**
 * 实例画廊：左侧菜单 · 右侧源码 + iframe 预览
 */

/**
 * @param {HTMLElement} root
 * @param {{
 *   title: string;
 *   description?: string;
 *   examples: Array<{
 *     id: string;
 *     title: string;
 *     description?: string;
 *     previewUrl: string;
 *     source: string;
 *   }>;
 *   defaultId?: string;
 * }} options
 */
export function mountExampleGallery(root, options) {
  const examples = options.examples;
  if (!examples.length) {
    throw new Error('examples 不能为空');
  }

  const ids = new Set(examples.map((item) => item.id));
  const pickInitial = () => {
    const hash = location.hash.replace(/^#/, '');
    if (hash && ids.has(hash)) return hash;
    if (options.defaultId && ids.has(options.defaultId)) return options.defaultId;
    return examples[0].id;
  };

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
        <pre class="example-source-code"><code></code></pre>
      </section>
      <section class="example-preview-panel">
        <div class="example-panel-header">预览</div>
        <iframe title="实例预览" loading="lazy"></iframe>
      </section>
    </div>
  `;

  const nav = root.querySelector('.example-nav');
  const codeEl = root.querySelector('.example-source-code code');
  const iframe = root.querySelector('iframe');
  const copyBtn = root.querySelector('[data-action="copy"]');

  /** @type {string} */
  let activeId = pickInitial();

  const getExample = (id) => examples.find((item) => item.id === id) ?? examples[0];

  const renderNav = () => {
    nav.innerHTML = examples
      .map(
        (item) => `
      <button
        type="button"
        class="example-nav-item${item.id === activeId ? ' is-active' : ''}"
        data-id="${escapeAttr(item.id)}"
      >
        <span class="example-nav-title">${escapeHtml(item.title)}</span>
        ${item.description ? `<span class="example-nav-desc">${escapeHtml(item.description)}</span>` : ''}
      </button>
    `,
      )
      .join('');
  };

  const selectExample = (id, { pushHash = true } = {}) => {
    if (!ids.has(id)) return;
    activeId = id;
    const example = getExample(id);

    renderNav();
    codeEl.textContent = example.source;
    iframe.src = example.previewUrl;

    if (pushHash && location.hash !== `#${id}`) {
      history.replaceState(null, '', `#${id}`);
    }
  };

  nav.addEventListener('click', (event) => {
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
