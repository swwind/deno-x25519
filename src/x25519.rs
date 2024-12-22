use crate::curve25519::FieldElem;

pub trait Ecdh {
    type SecretKey;
    type PublicKey;
    type SharedSecret;

    fn pubkey(seckey: &Self::SecretKey) -> Self::PublicKey;
    fn ecdh(seckey: &Self::SecretKey, pubkey: &Self::PublicKey) -> Self::SharedSecret;
}

pub struct X25519;

impl X25519 {
    fn scalarmult(scalar: &[u8; 32], base: FieldElem) -> FieldElem {
        let (mut a, mut b, mut c, mut d) = (
            FieldElem(FieldElem::_1),
            base.clone(),
            FieldElem(FieldElem::_0),
            FieldElem(FieldElem::_1),
        );
        let (mut e, mut f);

        for i in (0..255).rev() {
            // a = Xi, b = X{i+1}, c = Zi, d = Z{i+1}
            let bit = ((scalar[i >> 3] >> (i & 7)) & 1) == 1;

            // if bit == 1, we swap(a, b), swap(c, d) in order to make
            // a = X{i+1}, b = Xi, c = Z{i+1}, d = Z{i}
            a.swap(&mut b, bit);
            c.swap(&mut d, bit);

            // this period takes
            // a = Xi, b = X{i+1}, c = Zi, d = Z{i+1}
            // and gets result
            // a = X{2i}, b = X{2i+1}, c = Z{2i}, d = Z{2i+1}
            e = &a + &c;
            a = &a - &c;
            c = &b + &d;
            b = &b - &d;
            d = &e * &e;
            f = &a * &a;
            a = &c * &a;
            c = &b * &e;
            e = &a + &c;
            a = &a - &c;
            b = &a * &a;
            c = &d - &f;
            a = &c * &FieldElem(FieldElem::_121665);
            a = &a + &d;
            c = &c * &a;
            a = &d * &f;
            d = &b * &base;
            b = &e * &e;

            // if bit == 1, now we have
            // a = X{2i+2}, b = X{2i+1}, c = Z{2i+2}, d = Z{2i+1}
            // we need to swap them back
            a.swap(&mut b, bit);
            c.swap(&mut d, bit);
        }

        // returns X{x} / Z{x}
        &a * &c.inverse()
    }
}

impl Ecdh for X25519 {
    type SecretKey = [u8; 32];
    type PublicKey = [u8; 32];
    type SharedSecret = [u8; 32];

    fn pubkey(seckey: &Self::SecretKey) -> Self::PublicKey {
        Self::scalarmult(seckey, FieldElem(FieldElem::_9)).into_bytes()
    }

    fn ecdh(seckey: &Self::SecretKey, pubkey: &Self::PublicKey) -> Self::SharedSecret {
        Self::scalarmult(seckey, FieldElem::from_bytes(pubkey)).into_bytes()
    }
}
