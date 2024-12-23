import { ecdh as _ecdh, pubkey as _pubkey } from "./pkg/x25519.js";

/**
 * Generate a crypto-safe random SecretKey.
 */
export function genkey(): Uint8Array {
  const seckey = crypto.getRandomValues(new Uint8Array(32));
  seckey[0] &= 0xf8;
  seckey[31] = (seckey[31] & 0x7f) | 0x40;
  return seckey;
}

/**
 * Derive PublicKey from SecretKey. (wasm)
 */
export function pubkey(sk: Uint8Array): Uint8Array {
  return _pubkey(sk);
}

/**
 * Compute SharedSecret with given SecretKey and PublicKey. (wasm)
 */
export function ecdh(sk: Uint8Array, pk: Uint8Array): Uint8Array {
  return _ecdh(sk, pk);
}
