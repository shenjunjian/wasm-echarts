/**
 * 实例页公共 shell：左源码 · 右预览
 */
export function mountExample(root, options) {
  root.className = 'example-page';
  root.innerHTML = `
    <div class="example-toolbar">
      <div>
        <h1>${escapeHtml(options.title)}</h1>
        ${options.description ? `<div class="desc">${escapeHtml(options.description)}</div>` : ''}
      </div>
      <div class="example-toolbar-actions">
        <a class="btn" href="${escapeAttr(options.backHref)}">← 实例列表</a>
        <button type="button" class="btn" data-action="reset">重置</button>
        <button type="button" class="btn btn-primary" data-action="run">运行</button>
      </div>
    </div>
    <div class="example-split">
      <div class="example-source">
        <div class="example-source-header">源码</div>
        <textarea spellcheck="false" aria-label="源码编辑器"></textarea>
      </div>
      <div class="example-preview-wrap">
        <div class="example-preview-header">预览</div>
        <div class="example-preview">
          <div class="chart-host"></div>
        </div>
      </div>
    </div>
    <div class="example-log" data-log>就绪</div>
  `;

  const textarea = root.querySelector('textarea');
  const previewHost = root.querySelector('.chart-host');
  const logEl = root.querySelector('[data-log]');
  const defaultSource = options.defaultSource;

  textarea.value = defaultSource;

  /** @type {{ dispose?: () => void } | null} */
  let current = null;

  const log = (msg) => {
    logEl.textContent = msg;
  };

  const cleanup = () => {
    if (current?.dispose) {
      try {
        current.dispose();
      } catch (_) {
        /* ignore */
      }
    }
    current = null;
    previewHost.innerHTML = '';
  };

  const run = async () => {
    cleanup();
    log('运行中…');
    try {
      current = (await options.run(textarea.value, previewHost, log)) ?? null;
      if (logEl.textContent === '运行中…') {
        log('运行成功');
      }
    } catch (err) {
      log(`错误: ${err?.message ?? err}`);
      console.error(err);
    }
  };

  root.querySelector('[data-action="run"]').addEventListener('click', () => {
    void run();
  });

  root.querySelector('[data-action="reset"]').addEventListener('click', () => {
    textarea.value = defaultSource;
    void run();
  });

  void run();
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
