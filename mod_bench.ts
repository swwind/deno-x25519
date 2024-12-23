import { decodeBase64 } from "@std/encoding";
import { ecdh, pubkey } from "./mod.ts";

const sk = decodeBase64("YNeyOlGYBGTXyTWlaFQDCklszM2Veo1uYFoSsEYJ/kg=");
const pk = decodeBase64("fppOZbL71peOnZoV1l9/9OI28RcN60VJQiAh6JZvWEM=");

Deno.bench("js pubkey", () => {
  pubkey(sk);
});

Deno.bench("js ecdh", () => {
  ecdh(sk, pk);
});
