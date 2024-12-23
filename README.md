# x25519 for deno

A simple library for x25519 key exchange algorithm.

## Features

This library provides both JS implementation (with BigInt) and WASM
implementation, both contains

- `genkey`: Generate cryptographically secure random secret keys.
- `pubkey`: Derive public keys from secret keys.
- `ecdh`: Compute shared secrets for secure communication.

All those functions is as simple and stupid as you think.

| Difference      | JS     | WASM   |
| --------------- | ------ | ------ |
| Runs const-time | no     | yes    |
| Benchmark       | ~950us | ~560us |

## Installation

Install via [jsr](https://jsr.io/@swwind/x25519/) repository.

```sh
deno add jsr:@swwind/x25519
```

## Usage

- import from `@swwind/x25519` if you want the JS version.
- import from `@swwind/x25519/wasm` if you want the WASM version.

### Generate a Secret Key

```javascript
import { genkey } from "@swwind/x25519";

const sk = genkey();
console.log("Secret Key:", encodeBase64(sk));
// AIPkgkzYjSF85jOgO1V6+bM49VCWrwoPcwlKiqyisWY=
```

### Derive a Public Key

```javascript
import { pubkey } from "@swwind/x25519";

const pk = pubkey(sk);
console.log("Public Key:", encodeBase64(pk));
// fppOZbL71peOnZoV1l9/9OI28RcN60VJQiAh6JZvWEM=
```

### Compute a Shared Secret

```javascript
import { ecdh } from "@swwind/x25519";

const s = ecdh(sk, pk);
console.log("Shared Secret:", encodeBase64(s));
// eiSMV9fI1f/pTdB79E318wddCQhifZF2TAIlla0Cv0o=
```

## Notes

This WASM implementation should be const-time, but I did not test it so much. So
use it at your own risk.

By the way, WASM implementation requires SIMD feature to be enabled, this should
not be a big problem if you are not come from last century.

## Benchmark

```
    CPU | AMD Ryzen 7 4800H with Radeon Graphics
Runtime | Deno 2.0.5 (x86_64-unknown-linux-gnu)

file:///home/swwind/Repo/work/deno-x25519/mod_bench.ts

benchmark   time/iter (avg)        iter/s      (min … max)           p75      p99     p995
----------- ----------------------------- --------------------- --------------------------
js pubkey          901.4 µs         1,109 (845.4 µs …   1.2 ms) 880.2 µs   1.1 ms   1.1 ms
js ecdh            949.9 µs         1,053 (895.2 µs …   2.0 ms) 937.3 µs   1.2 ms   1.3 ms


file:///home/swwind/Repo/work/deno-x25519/wasm_bench.ts

benchmark     time/iter (avg)        iter/s      (min … max)           p75      p99     p995
------------- ----------------------------- --------------------- --------------------------
wasm pubkey          561.2 µs         1,782 (544.8 µs … 644.6 µs) 565.9 µs 580.0 µs 591.4 µs
wasm ecdh            560.9 µs         1,783 (542.5 µs … 736.8 µs) 566.8 µs 578.9 µs 585.5 µs
```

## License

MIT
