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
      '@shared': resolve(root, 'src/shared'),
      '@zrender': resolve(root, 'src/zrender'),
      '@echarts': resolve(root, 'src/echarts'),
      '@wasm-zrender': resolve(repoRoot, 'crates/wasm-zrender/pkg'),
      '@wasm-echarts': resolve(repoRoot, 'crates/wasm-echarts/pkg'),
    },
  },
  assetsInclude: ['**/*.wasm'],
  optimizeDeps: {
    exclude: ['@wasm-zrender/wasm_zrender.js', '@wasm-echarts/wasm_echarts.js'],
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
