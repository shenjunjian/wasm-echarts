/**
 * 兼容旧入口：等价于 probe-official-examples.mjs --category line
 */
import { main } from './probe-official-examples.mjs';

main(['--category', 'line', ...process.argv.slice(2)]).catch((err) => {
  console.error(err);
  process.exit(1);
});
