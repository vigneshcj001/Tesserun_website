import { spawn, spawnSync } from 'node:child_process';
import { readFile, writeFile, lstat } from 'node:fs/promises';
import { resolve, sep } from 'node:path';
import { env, root, trunk, cargo, prepareRust } from './rust-tools.mjs';
import { sourceHash } from './source-hash.mjs';

const mode = process.argv[2] || 'build';
prepareRust();
const commands = {
  lock: [cargo, ['generate-lockfile']],
  build: [trunk, ['build', '--release', '--locked']],
  dev: [trunk, ['serve', '--locked']],
  check: [cargo, ['check', '--target', 'wasm32-unknown-unknown', '--locked']],
  format: [cargo, ['fmt', '--all']],
  lint: [cargo, ['clippy', '--target', 'wasm32-unknown-unknown', '--locked', '--', '-D', 'warnings']],
  test: [cargo, ['test', '--locked']],
};
if (!commands[mode]) throw new Error(`Unknown action: ${mode}`);
async function cleanOutput() {
  // Remove only this checkout's generated output, including interrupted stages.
  const output = resolve(root, 'dist');
  if (!output.startsWith(resolve(root) + sep)) throw new Error('Output must stay inside this checkout');
  const info = await lstat(output).catch(error => {
    if (error.code === 'ENOENT') return null;
    throw error;
  });
  if (info?.isSymbolicLink()) throw new Error('Refusing to clean a linked output directory');
  const clean = spawnSync(trunk, ['clean', '--dist', output], { cwd: root, env, stdio: 'inherit' });
  if (clean.status !== 0) process.exit(clean.status ?? 1);
}
const [command, args] = commands[mode];
function run() {
  return new Promise(resolveRun => {
    let stderr = '';
    const child = spawn(command, args, { cwd: root, env, stdio: ['inherit', 'pipe', 'pipe'] });
    child.stdout.on('data', data => { stderr = (stderr + data).slice(-64000); process.stdout.write(data); });
    child.stderr.on('data', data => { stderr = (stderr + data).slice(-64000); process.stderr.write(data); });
    child.on('error', error => resolveRun({ error, status: 1, stderr }));
    child.on('close', status => resolveRun({ status, stderr }));
  });
}
let result;
for (let attempt = 0; attempt < 3; attempt++) {
  if (mode === 'build') await cleanOutput();
  result = await run();
  const stagingConflict = process.platform === 'win32' && mode === 'build'
    && result.stderr.includes('error applying built distribution');
  if (!stagingConflict || result.status === 0 || attempt === 2) break;
  console.error('Retrying the Windows staging operation after a transient file lock...');
  await new Promise(resolveDelay => setTimeout(resolveDelay, 1000));
}
if (result.error) {
  console.error(`${result.error.message}\nInstall Rust, its wasm32-unknown-unknown target, and Trunk 0.22.1. See README.md.`);
}
if (mode === 'build' && result.status === 0) {
  const indexPath = resolve(root, 'dist/index.html');
  const html = await readFile(indexPath, 'utf8');
  await writeFile(indexPath, html.replace(/[\t ]+$/gm, '').trimEnd() + '\n');
  await writeFile(resolve(root, 'dist/build-info.json'), JSON.stringify({ sourceHash: await sourceHash(), stack: 'Leptos 0.8.23 / Trunk 0.22.1' }, null, 2) + '\n');
}
process.exit(result.status ?? 1);
