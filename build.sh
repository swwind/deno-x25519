# build
wasm-pack build --target deno

# fix imports for nodejs
sed -i '/wasmCode/{N;N;N;N;N;N;N;N;N;N;N;s/.*/const wasmCode = await (await fetch(wasm_url)).arrayBuffer();/}' pkg/x25519.js
