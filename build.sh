# build
wasm-pack build --target deno

# fix imports for nodejs
sed -i '1s!^!import { readFile } from "node:fs/promises";!' pkg/x25519.js
sed -i 's/await Deno.readFile/await readFile/' pkg/x25519.js
