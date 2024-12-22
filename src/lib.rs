use wasm_bindgen::prelude::*;
use x25519::{Ecdh, X25519};

mod curve25519;
mod x25519;

#[wasm_bindgen]
pub fn pubkey(seckey: &[u8]) -> Box<[u8]> {
    let seckey = seckey.try_into().unwrap();
    let pubkey = X25519::pubkey(&seckey);
    Box::new(pubkey)
}

#[wasm_bindgen]
pub fn x25519(seckey: &[u8], pubkey: &[u8]) -> Box<[u8]> {
    let seckey = seckey.try_into().unwrap();
    let pubkey = pubkey.try_into().unwrap();
    let shared = X25519::ecdh(&seckey, &pubkey);
    Box::new(shared)
}
