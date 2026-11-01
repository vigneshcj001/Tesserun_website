import { createHash } from 'node:crypto';
import { readdir, readFile } from 'node:fs/promises';
import { resolve, relative } from 'node:path';
import { root } from './rust-tools.mjs';

export async function sourceHash() {
  const files = ['Cargo.toml', 'Cargo.lock', 'Trunk.toml', 'index.html', 'styles.css', 'experience.css'];
  async function collect(directory) {
    for (const entry of await readdir(directory, { withFileTypes: true })) {
      const path = resolve(directory, entry.name);
      if (entry.isDirectory()) await collect(path);
      else files.push(relative(root, path).replaceAll('\\', '/'));
    }
  }
  await collect(resolve(root, 'src'));
  await collect(resolve(root, 'public'));
  const hash = createHash('sha256');
  for (const file of files.sort()) {
    const bytes = await readFile(resolve(root, file));
    // Git may normalize text line endings across Windows and Linux checkouts.
    const content = /\.(toml|lock|html|css|rs|svg|txt|json|m?js)$/i.test(file)
      ? bytes.toString('utf8').replaceAll('\r\n', '\n').replace(/^\uFEFF/, '')
      : bytes;
    hash.update(file).update('\0').update(content).update('\0');
  }
  return hash.digest('hex');
}
