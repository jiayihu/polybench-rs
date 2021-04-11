import fs from 'fs';
import { promisify } from 'node:util';
import { exec } from 'child_process';

const readdir = promisify(fs.readdir);
const readFile = promisify(fs.readFile);
const writeFile = promisify(fs.writeFile);
const shell_exec = promisify(exec);

async function benchNames() {
  const files = await readdir('./src');

  return files.map((file) => file.replace('.rs', '')).filter((name) => name !== 'utils');
}

async function compile() {
  const names = await benchNames();

  for (const name of names) {
    let content = await readFile('./Cargo.toml', 'utf-8');

    content = content.replace(/name = "(\w+)"/, `name = "${name}"`);
    content = content.replace(/path = "src\/(\w+).rs"/, `path = "src/${name}.rs"`);

    await writeFile('./Cargo.toml', content, 'utf-8');

    // await shell_exec(`cargo make build-wasm ${name}`);
    await shell_exec(
      `CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-unknown-linux-gnu-gcc cargo build --lib --target x86_64-unknown-linux-gnu --release`,
    );
  }
}

async function interpret() {
  const names = await benchNames();

  for (const name of names) {
    await shell_exec(`wasm-interp wasm/${name}.wasm --run-all-exports --trace`, {
      maxBuffer: 10 * 1024 * 1024,
    });

    console.log('Interpreted', name);
  }
}

compile();

// interpret();
