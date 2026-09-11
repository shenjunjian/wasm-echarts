/**
 * 触发仓库里的 Deploy pages workflow（CI 用已提交的 pkg/ 编 site，不提交 dist）。
 * 需要已安装并登录 GitHub CLI：https://cli.github.com/
 */

import { spawnSync } from 'node:child_process';

function run(command, args) {
  const result = spawnSync(command, args, {
    stdio: 'inherit',
    shell: true,
  });
  return result.status ?? 1;
}

if (run('gh', ['--version']) !== 0) {
  console.error('未找到 GitHub CLI (gh)。请安装并执行 gh auth login：https://cli.github.com/');
  process.exit(1);
}

const status = run('gh', ['workflow', 'run', 'deploy-pages.yml', '--ref', 'main']);
if (status !== 0) {
  process.exit(status);
}

console.log('\n已触发 Deploy pages。查看进度：');
console.log('  gh run list --workflow=deploy-pages.yml');
console.log('  https://github.com/shenjunjian/wasm-echarts/actions');
