use std::{
    mem,
    ops::{Add, Mul, Sub},
};

#[cfg(target_endian = "big")]
compile_error!("This code does not support big-endian architectures.");

#[derive(Debug, Clone)]
pub struct FieldElem(pub [i64; 16]);

impl FieldElem {
    pub const _0: [i64; 16] = [0; 16];
    pub const _1: [i64; 16] = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    pub const _9: [i64; 16] = [9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    pub const _121665: [i64; 16] = [56129, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    pub const _25519: [i64; 16] = [
        0xffed, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff,
        0xffff, 0xffff, 0xffff, 0xffff, 0x7fff,
    ];
}

impl FieldElem {
    pub fn from_bytes(bytes: &[u8; 32]) -> Self {
        let elem: [u16; 16] = unsafe { mem::transmute(*bytes) };
        let mut array = [0i64; 16];
        for i in 0..16 {
            array[i] = elem[i] as i64;
        }
        Self(array)
    }

    pub fn into_bytes(mut self) -> [u8; 32] {
        self.normalize();
        let mut array = [0u16; 16];
        for i in 0..16 {
            array[i] = self.0[i] as u16;
        }
        unsafe { mem::transmute(array) }
    }
}

impl FieldElem {
    pub fn carry(&mut self) {
        for i in 0..16 {
            let carry = self.0[i] >> 16;
            self.0[i] -= carry << 16;
            if i != 15 {
                self.0[i + 1] += carry;
            } else {
                self.0[0] += 38 * carry;
            }
        }
    }

    /// self ^ {-1}
    pub fn inverse(&self) -> Self {
        let mut tmp = self.clone();
        for i in (0..254).rev() {
            tmp = &tmp * &tmp;
            if i != 2 && i != 4 {
                tmp = &tmp * &self;
            }
        }
        tmp
    }

    /// \sqrt{self}
    ///
    /// = self ^ {(p+3)/8}
    pub fn residue(&self) -> Self {
        let mut tmp = self.clone();
        for _ in 0..250 {
            tmp = &tmp * &tmp;
            tmp = &tmp * &self;
        }
        &tmp * &tmp
    }

    /// swap two elements if swap == 1.
    pub fn swap(&mut self, rhs: &mut FieldElem, swap: bool) {
        let c = if swap { -1 } else { 0 };
        for i in 0..16 {
            let t = c & (self.0[i] ^ rhs.0[i]);
            self.0[i] ^= t;
            rhs.0[i] ^= t;
        }
    }

    /// make self in [0, p-1]
    pub fn normalize(&mut self) {
        self.carry();
        self.carry();
        self.carry();

        // then t will be [0, 2^256-1]
        // we need to calculate {t, t-p, t-2p} and determine which is our result
        for _ in 0..2 {
            // we first calculate m = t - p
            // and swap self and m if m > 0
            let mut m = FieldElem(FieldElem::_0);
            let mut carry = 0;
            for i in 0..16 {
                m.0[i] = self.0[i] - FieldElem::_25519[i] - carry;
                carry = (m.0[i] >> 16) & 1;
                if i > 0 {
                    m.0[i - 1] &= 0xffff;
                }
            }
            // we need to make t = m if m >= 0
            self.swap(&mut m, carry == 0);
        }
    }
}

impl Add<&FieldElem> for &FieldElem {
    type Output = FieldElem;

    fn add(self, rhs: &FieldElem) -> Self::Output {
        FieldElem(core::array::from_fn(|i| self.0[i] + rhs.0[i]))
    }
}

impl Sub<&FieldElem> for &FieldElem {
    type Output = FieldElem;

    fn sub(self, rhs: &FieldElem) -> Self::Output {
        FieldElem(core::array::from_fn(|i| self.0[i] - rhs.0[i]))
    }
}

impl Mul<&FieldElem> for &FieldElem {
    type Output = FieldElem;

    fn mul(self, rhs: &FieldElem) -> Self::Output {
        let mut array = [0i64; 32];

        for i in 0..16 {
            for j in 0..16 {
                array[i + j] += self.0[i] * rhs.0[j];
            }
        }

        for i in 0..16 {
            array[i] += 38 * array[i + 16];
        }

        let mut elem = FieldElem(array[0..16].try_into().unwrap());
        elem.carry();
        elem.carry();
        elem
    }
}
