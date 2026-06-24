/**
 * iframe 预览页：挂载 chart-host 并运行实例
 * @param {(host: HTMLElement, log: (msg: string) => void) => Promise<{ dispose?: () => void } | void>} run
 */
export async function mountPreview(run) {
  const root = document.getElementById('app');
  root.className = 'preview-root';

  const host = document.createElement('div');
  host.className = 'chart-host';
  root.appendChild(host);

  const logEl = document.createElement('div');
  logEl.className = 'preview-log';
  logEl.textContent = '加载中…';
  document.body.appendChild(logEl);

  const log = (msg) => {
    logEl.textContent = msg;
  };

  try {
    await run(host, log);
    if (logEl.textContent === '加载中…') {
      log('就绪');
    }
  } catch (err) {
    log(`错误: ${err?.message ?? err}`);
    console.error(err);
  }
}
