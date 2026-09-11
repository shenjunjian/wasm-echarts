import { readdirSync, statSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { defineConfig } from 'vite';

const root = fileURLToPath(new URL('.', import.meta.url));
const repoRoot = resolve(root, '..');

/** GitHub Pages 项目站传入 `/wasm-echarts`；本地开发保持 `/`。 */
function viteBase() {
  const raw = process.env.SITE_BASE;
  if (!raw || raw === '/') return '/';
  return raw.endsWith('/') ? raw : `${raw}/`;
}

/** 构建后给 HTML 里仍写着的站内绝对路径补上 base。 */
function rewriteAbsoluteUrls(base) {
  const prefix = (base || '/').replace(/\/$/, '');
  return {
    name: 'rewrite-absolute-urls',
    transformIndexHtml: {
      order: 'post',
      handler(html) {
        if (!prefix) return html;
        return html.replace(/\b(href|src)="(\/[^"]*)"/g, (full, attr, url) => {
          if (url === prefix || url.startsWith(`${prefix}/`)) return full;
          if (url === '/') return `${attr}="${prefix}/"`;
          return `${attr}="${prefix}${url}"`;
        });
      },
    },
  };
}

/** @returns {Record<string, string>} */
function collectHtmlEntries(dir = root, acc = {}) {
  for (const name of readdirSync(dir)) {
    if (name === 'node_modules' || name === 'dist' || name === 'src') continue;
    const full = join(dir, name);
    if (statSync(full).isDirectory()) {
      collectHtmlEntries(full, acc);
    } else if (name.endsWith('.html')) {
      const key = relative(root, full).replace(/\\/g, '/').replace(/\.html$/, '') || 'index';
      acc[key] = full;
    }
  }
  return acc;
}

const COOP_COEP_HEADERS = {
  'Cross-Origin-Opener-Policy': 'same-origin',
  'Cross-Origin-Embedder-Policy': 'require-corp',
};

const siteBase = viteBase();

export default defineConfig({
  root,
  base: siteBase,
  server: {
    fs: { allow: [repoRoot] },
    headers: COOP_COEP_HEADERS,
  },
  preview: {
    headers: COOP_COEP_HEADERS,
  },
  resolve: {
    alias: {
      '@wasm-zrender': resolve(repoRoot, 'crates/wasm-zrender/js'),
      '@wasm-echarts': resolve(repoRoot, 'crates/wasm-echarts/js'),
    },
  },
  assetsInclude: ['**/*.wasm'],
  optimizeDeps: {
    exclude: ['@wasm-zrender', '@wasm-echarts'],
  },
  plugins: [
    rewriteAbsoluteUrls(siteBase),
    {
      name: 'wasm-mime',
      configureServer(server) {
        server.middlewares.use((req, res, next) => {
          if (req.url?.includes('.wasm')) {
            res.setHeader('Content-Type', 'application/wasm');
          }
          next();
        });
      },
    },
  ],
  build: {
    target: 'esnext',
    rollupOptions: {
      input: collectHtmlEntries(),
    },
  },
});
