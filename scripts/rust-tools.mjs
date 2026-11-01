import { existsSync, statSync, mkdirSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { resolve, delimiter, dirname } from 'node:path';

export const root = fileURLToPath(new URL('../', import.meta.url));
export const env = { ...process.env };
const windows = process.platform === 'win32';
const siblingTools = resolve(root, '../Tesserun/.tools');
const localCargo = resolve(siblingTools, 'cargo/bin/cargo.exe');

// Reuse this workspace's existing toolchain; fresh clones use system rustup.
if (windows && existsSync(localCargo)) {
  env.CARGO_HOME = resolve(siblingTools, 'cargo');
  env.RUSTUP_HOME = resolve(siblingTools, 'rustup');
  env.RUSTUP_TOOLCHAIN = 'stable-x86_64-pc-windows-gnu';
  env.CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER = 'ld.lld.exe';
  env.RUSTC_WRAPPER = resolve(root, '.tools/bin/rustc-wrapper.exe');
  const originalPath = Object.entries(env).find(([key]) => key.toLowerCase() === 'path')?.[1] || '';
  for (const key of Object.keys(env)) if (key.toLowerCase() === 'path') delete env[key];
  env.PATH = [
    dirname(process.execPath),
    resolve(siblingTools, 'cargo/bin'),
    resolve(siblingTools, 'llvm-mingw/llvm-mingw-20261020-ucrt-x86_64/bin'),
    resolve(siblingTools, 'rustup/toolchains/stable-x86_64-pc-windows-gnu/lib/rustlib/x86_64-pc-windows-gnu/bin'),
    originalPath,
  ].join(delimiter);
}

const localTrunk = resolve(root, '.tools/bin', windows ? 'trunk.exe' : 'trunk');
export const trunk = existsSync(localTrunk) ? localTrunk : 'trunk';
export const cargo = windows && existsSync(localCargo) ? localCargo : 'cargo';

export function prepareRust() {
  if (!env.RUSTC_WRAPPER?.endsWith('rustc-wrapper.exe')) return;
  const source = resolve(root, 'scripts/rustc-wrapper.rs');
  if (existsSync(env.RUSTC_WRAPPER) && statSync(env.RUSTC_WRAPPER).mtimeMs >= statSync(source).mtimeMs) return;
  mkdirSync(resolve(root, '.tools/bin'), { recursive: true });
  const compiler = resolve(siblingTools, 'rustup/toolchains/stable-x86_64-pc-windows-gnu/bin/rustc.exe');
  const result = spawnSync(compiler, [source, '--crate-name', 'tesserun_rustc_wrapper', '--edition=2024', '-O', '-o', env.RUSTC_WRAPPER,
    '-Clinker=ld.lld.exe', '-Clinker-flavor=ld', '-Clink-self-contained=yes', '-Cdlltool=llvm-dlltool.exe'], { env, stdio: 'inherit' });
  if (result.status !== 0) throw new Error('Could not prepare the local GNU cross-compilation helper');
}
