# x25519 for deno

A simple library for x25519 key exchange algorithm.

## Features

This library provides both JS implementation (with BigInt) and WASM
implementation, both contains

- `genkey`: Generate cryptographically secure random secret keys.
- `pubkey`: Derive public keys from secret keys.
- `ecdh`: Compute shared secrets for secure communication.

All those functions is as simple and stupid as you think.

| Difference      | JS   | WASM   |
| --------------- | ---- | ------ |
| Runs const-time | no   | yes    |
| Benchmark       | ~1ms | ~650us |

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

This implementation should be const-time, but I did not test it so much. So use
it at your own risk.

## Benchmark

```
    CPU | AMD Ryzen 7 4800H with Radeon Graphics
Runtime | Deno 2.0.5 (x86_64-unknown-linux-gnu)

file:///home/swwind/Repo/work/deno-x25519/mod_bench.ts

benchmark   time/iter (avg)        iter/s      (min … max)           p75      p99     p995
----------- ----------------------------- --------------------- --------------------------
js pubkey          916.9 µs         1,091 (847.4 µs …   1.2 ms) 978.6 µs   1.1 ms   1.1 ms
js ecdh            944.4 µs         1,059 (882.4 µs …   1.1 ms)   1.0 ms   1.1 ms   1.1 ms


file:///home/swwind/Repo/work/deno-x25519/wasm_bench.ts

benchmark     time/iter (avg)        iter/s      (min … max)           p75      p99     p995
------------- ----------------------------- --------------------- --------------------------
wasm pubkey          646.7 µs         1,546 (629.8 µs … 742.1 µs) 647.2 µs 735.0 µs 739.6 µs
wasm ecdh            644.6 µs         1,551 (630.8 µs … 831.8 µs) 646.7 µs 710.2 µs 740.1 µs
```

## License

MIT
