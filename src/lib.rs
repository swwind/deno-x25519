use curve25519::FieldElem;
use wasm_bindgen::prelude::*;

mod curve25519;
mod x25519;

#[wasm_bindgen]
pub fn scalarmult(seckey: &[u8], pubkey: &[u8]) -> Box<[u8]> {
    let seckey = seckey.try_into().unwrap();
    let pubkey = pubkey.try_into().unwrap();
    let shared = x25519::scalarmult(&seckey, FieldElem::from_bytes(&pubkey)).into_bytes();
    Box::new(shared)
}
