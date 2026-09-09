import { readdirSync, statSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { defineConfig } from 'vite';

const root = fileURLToPath(new URL('.', import.meta.url));
const repoRoot = resolve(root, '..');

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

export default defineConfig({
  root,
  server: {
    fs: { allow: [repoRoot] },
  },
  resolve: {
    alias: {
      '@wasm-zrender': resolve(repoRoot, 'crates/wasm-zrender/js'),
      '@wasm-echarts': resolve(repoRoot, 'crates/wasm-echarts/pkg/wasm_echarts.js'),
    },
  },
  assetsInclude: ['**/*.wasm'],
  optimizeDeps: {
    exclude: ['@wasm-zrender', '@wasm-echarts'],
  },
  plugins: [
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
