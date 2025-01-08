import { clamp } from "./utils.ts";

/**
 * Generate a crypto-safe random SecretKey.
 */
export function genkey(): Uint8Array {
  const seckey = crypto.getRandomValues(new Uint8Array(32));
  return clamp(seckey);
}

const p = (1n << 255n) - 19n;

function fastpow(x: bigint, y: bigint) {
  let z = 1n;
  for (; y > 0n; y >>= 1n, x = (x * x) % p) {
    if (y & 1n) z = (z * x) % p;
  }
  return z;
}

function inverse(x: bigint) {
  return fastpow(x, p - 2n);
}

function scalarmult(scalar: Uint8Array, base: bigint) {
  let a = 1n,
    b = base,
    c = 0n,
    d = 1n,
    e: bigint,
    f: bigint;
  for (let i = 254; i >= 0; --i) {
    const bit = (scalar[i >> 3] >> (i & 7)) & 1;
    [a, b, c, d] = bit ? [b, a, d, c] : [a, b, c, d];
    e = (a + c) % p;
    a = (a - c + p) % p;
    c = (b + d) % p;
    b = (b - d + p) % p;
    d = (e * e) % p;
    f = (a * a) % p;
    a = (c * a) % p;
    c = (b * e) % p;
    e = (a + c) % p;
    a = (a - c + p) % p;
    b = (a * a) % p;
    c = (d - f + p) % p;
    a = (c * 121665n) % p;
    a = (a + d) % p;
    c = (c * a) % p;
    a = (d * f) % p;
    d = (b * base) % p;
    b = (e * e) % p;
    [a, b, c, d] = bit ? [b, a, d, c] : [a, b, c, d];
  }
  return (a * inverse(c)) % p;
}

function unpack(x: bigint) {
  const arr = new Uint8Array(32);
  for (let i = 0; i < 32; ++i) {
    arr[i] = Number((x >> BigInt(i * 8)) & 0xffn);
  }
  return arr;
}

function pack(buffer: Uint8Array) {
  let result = 0n;
  for (let i = 31; i >= 0; --i) {
    result = result * 256n + BigInt(buffer[i]);
  }
  return result;
}

/**
 * Derive PublicKey from SecretKey. (js)
 */
export function pubkey(sk: Uint8Array): Uint8Array {
  return unpack(scalarmult(clamp(sk), 9n));
}

/**
 * Compute SharedSecret with given SecretKey and PublicKey. (js)
 */
export function ecdh(sk: Uint8Array, pk: Uint8Array): Uint8Array {
  return unpack(scalarmult(clamp(sk), pack(pk)));
}
