import { pubkey as _pubkey, x25519 as _x25519 } from "./pkg/x25519.js";

/**
 * Generate a crypto-safe random SecretKey.
 */
export function genkey(): Uint8Array {
  const seckey = crypto.getRandomValues(new Uint8Array(32));
  seckey[0] &= 0xf8;
  seckey[31] &= 0x7f;
  seckey[31] |= 0x40;
  return seckey;
}

/**
 * Derive PublicKey from SecretKey.
 */
export function pubkey(sk: Uint8Array): Uint8Array {
  return _pubkey(sk);
}

/**
 * Compute SharedSecret with given SecretKey and PublicKey.
 */
export function x25519(sk: Uint8Array, pk: Uint8Array): Uint8Array {
  return _x25519(sk, pk);
}
