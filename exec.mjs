import fs from 'fs';
import { promisify } from 'node:util';
import { exec } from 'child_process';

const readdir = promisify(fs.readdir);
const readFile = promisify(fs.readFile);
const writeFile = promisify(fs.writeFile);
const shell = promisify(exec);

async function main() {
  const files = await readdir('./src');

  for (const file of files) {
    const name = file.replace('.rs', '');

    if (name === 'utils') continue;

    let content = await readFile('./Cargo.toml', 'utf-8');

    content = content.replace(/name = "(\w+)"/, `name = "${name}"`);
    content = content.replace(/path = "src\/(\w+).rs"/, `path = "src/${name}.rs"`);

    await writeFile('./Cargo.toml', content, 'utf-8');
    // await shell(`cargo make build-wasm ${name}`);
    await shell(
      `CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER cargo build --lib --target x86_64-unknown-linux-gnu --release`,
    );
  }
}

main();
