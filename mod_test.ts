import { assertEquals } from "@std/assert";
import { genkey, pubkey, x25519 } from "./mod.ts";
import { decodeBase64, encodeBase64 } from "@std/encoding";

function checkKeyPair(sk: string, pk: string) {
  assertEquals(encodeBase64(pubkey(decodeBase64(sk))), pk);
}

Deno.test("pubkey test", () => {
  checkKeyPair(
    "YNeyOlGYBGTXyTWlaFQDCklszM2Veo1uYFoSsEYJ/kg=",
    "IM+/Mi2gf7FEUtSRQLjqvx+R+xMj/+ClMquN/XmKKmg=",
  );
  checkKeyPair(
    "AIPkgkzYjSF85jOgO1V6+bM49VCWrwoPcwlKiqyisWY=",
    "fppOZbL71peOnZoV1l9/9OI28RcN60VJQiAh6JZvWEM=",
  );
});

Deno.test("x25519 test", () => {
  const sk1 = genkey();
  const pk1 = pubkey(sk1);

  const sk2 = genkey();
  const pk2 = pubkey(sk2);

  const s1 = x25519(sk1, pk2);
  const s2 = x25519(sk2, pk1);

  assertEquals(encodeBase64(s1), encodeBase64(s2));
});
