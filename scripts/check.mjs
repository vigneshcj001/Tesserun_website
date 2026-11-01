import { readFile, access, readdir } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';
import { resolve } from 'node:path';
import assert from 'node:assert/strict';
import { sourceHash } from './source-hash.mjs';

const root = fileURLToPath(new URL('../dist/', import.meta.url));
const html = await readFile(resolve(root, 'index.html'), 'utf8');
const ids = [...html.matchAll(/\bid="([^"]+)"/g)].map(match => match[1]);
assert.equal(ids.length, new Set(ids).size, 'HTML IDs must be unique');
for (const [, reference] of html.matchAll(/(?:href|src)="([^"]+)"/g)) {
  if (/^https?:/.test(reference) || reference === '#') continue;
  if (reference.startsWith('#')) assert(ids.includes(reference.slice(1)), `Missing anchor: ${reference}`);
  else await access(resolve(root, reference.replace(/^\//, '')));
}
for (const [, name] of html.matchAll(/aria-(?:controls|labelledby)="([^"]+)"/g)) {
  assert(ids.includes(name), `Missing ARIA target: ${name}`);
}
const exe = await readFile(resolve(root, 'downloads/tesserun-0.3.0-windows-x64.exe'));
assert.equal(exe.subarray(0, 2).toString(), 'MZ', 'Download must be a Windows executable');
const hash = createHash('sha256').update(exe).digest('hex');
const sums = await readFile(resolve(root, 'downloads/SHA256SUMS.txt'), 'utf8');
assert(sums.includes(`${hash}  tesserun-0.3.0-windows-x64.exe`), 'Download checksum must match');
const wasmFiles = (await readdir(root)).filter(file => file.endsWith('.wasm'));
assert.equal(wasmFiles.length, 1, 'Exactly one compiled Rust WASM module must be present');
const wasm = await readFile(resolve(root, wasmFiles[0]));
assert.deepEqual([...wasm.subarray(0, 4)], [0, 97, 115, 109], 'WASM binary header must be valid');
assert(WebAssembly.validate(wasm), 'WebAssembly module must validate');
assert(html.includes(wasmFiles[0]), 'Built HTML must load the compiled WASM module');
assert(!html.includes('src="app.js"'), 'Legacy JavaScript must not be loaded');
const buildInfo = JSON.parse(await readFile(resolve(root, 'build-info.json'), 'utf8'));
assert.equal(buildInfo.sourceHash, await sourceHash(), 'Source changed after the last release build. Rebuild dist before publishing.');
console.log('Passed: local assets, anchors, unique IDs, ARIA targets, executable checksum, and compiled Rust WASM.');
