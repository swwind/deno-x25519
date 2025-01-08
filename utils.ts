export function clamp(sk: Uint8Array): Uint8Array {
  sk[0] &= 0xf8;
  sk[31] = (sk[31] & 0x7f) | 0x40;
  return sk;
}
