import { decodeBase64 } from "@std/encoding";
import { ecdh, genkey, pubkey } from "./wasm.ts";

const sk = decodeBase64("YNeyOlGYBGTXyTWlaFQDCklszM2Veo1uYFoSsEYJ/kg=");
const pk = decodeBase64("fppOZbL71peOnZoV1l9/9OI28RcN60VJQiAh6JZvWEM=");

Deno.bench("wasm pubkey", () => {
  pubkey(sk);
});

Deno.bench("wasm ecdh", () => {
  ecdh(sk, pk);
});

Deno.bench("wasm pubkey random", () => {
  pubkey(genkey());
});

Deno.bench("wasm ecdh random", () => {
  ecdh(genkey(), pk);
});
