import { scalarmult } from "./pkg/x25519.js";
import { clamp } from "./utils.ts";

/**
 * Generate a crypto-safe random SecretKey.
 */
export function genkey(): Uint8Array {
  const seckey = crypto.getRandomValues(new Uint8Array(32));
  return clamp(seckey);
}

const BASE = new Uint8Array(32);
BASE[0] = 9;

/**
 * Derive PublicKey from SecretKey. (wasm)
 */
export function pubkey(sk: Uint8Array): Uint8Array {
  return scalarmult(clamp(sk), BASE);
}

/**
 * Compute SharedSecret with given SecretKey and PublicKey. (wasm)
 */
export function ecdh(sk: Uint8Array, pk: Uint8Array): Uint8Array {
  return scalarmult(clamp(sk), pk);
}
