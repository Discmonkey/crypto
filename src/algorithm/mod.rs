use crate::bitstring::bytes::Bytes;

pub mod single_byte_xor;
pub mod repeating_xor;

pub trait Algorithm {
    fn encrypt(&self, message: &Bytes, key: &Bytes) -> Bytes;
    fn decrypt(&self, cipher_text: &Bytes, key: &Bytes) -> Bytes;
}