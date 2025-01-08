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
    CPU | Intel(R) Core(TM) i7-14700F
Runtime | Deno 2.1.4 (x86_64-unknown-linux-gnu)

file:///home/swwind/Work/deno-x25519/mod_bench.ts

benchmark          time/iter (avg)        iter/s      (min … max)           p75      p99     p995
------------------ ----------------------------- --------------------- --------------------------
js pubkey                 599.0 µs         1,669 (582.0 µs … 680.3 µs) 596.8 µs 642.0 µs 655.6 µs
js ecdh                   620.8 µs         1,611 (603.8 µs … 701.0 µs) 624.0 µs 676.3 µs 681.8 µs
js pubkey random          602.2 µs         1,661 (579.8 µs … 905.6 µs) 600.2 µs 794.8 µs 804.4 µs
js ecdh random            621.2 µs         1,610 (601.3 µs …   1.2 ms) 620.9 µs 676.4 µs 688.3 µs


file:///home/swwind/Work/deno-x25519/wasm_bench.ts

benchmark            time/iter (avg)        iter/s      (min … max)           p75      p99     p995
-------------------- ----------------------------- --------------------- --------------------------
wasm pubkey                 468.7 µs         2,133 (463.7 µs … 793.2 µs) 469.4 µs 474.5 µs 480.2 µs
wasm ecdh                   469.7 µs         2,129 (465.5 µs … 674.5 µs) 470.4 µs 474.7 µs 475.3 µs
wasm pubkey random          469.4 µs         2,130 (465.1 µs … 490.9 µs) 470.5 µs 474.9 µs 475.9 µs
wasm ecdh random            470.1 µs         2,127 (466.7 µs … 506.8 µs) 471.2 µs 476.8 µs 481.9 µs
```

## License

MIT
