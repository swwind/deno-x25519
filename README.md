# x25519 for deno

A simple library for x25519 key exchange using WebCrypto and WebAssembly.

## Features

- Generate cryptographically secure random secret keys.
- Derive public keys from secret keys.
- Compute shared secrets for secure communication.

As simple and stupid as you think.

## Installation

Install via [jsr](https://jsr.io/@swwind/x25519/) repository.

```sh
deno add jsr:@swwind/x25519
```

## Usage

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
import { x25519 } from "@swwind/x25519";

const s = x25519(sk, pk);
console.log("Shared Secret:", encodeBase64(s));
// eiSMV9fI1f/pTdB79E318wddCQhifZF2TAIlla0Cv0o=
```

## Notes

This implementation should be const-time, but I did not test it so much. So use it at your own risk.

## License

MIT
