use crate::bitstring::bytes::Bytes;

pub mod single_bit_xor;

pub trait Algorithm {
    fn encrypt(&self, message: &Bytes, key: &Bytes) -> Bytes;
    fn decrypt(&self, cipher_text: &Bytes, key: &Bytes) -> Bytes;
}