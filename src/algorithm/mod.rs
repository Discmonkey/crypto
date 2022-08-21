use crate::bitstring::bytes::Bytes;

pub mod single_bit_xor;

trait Algorithm {
    fn encrypt(message: &Bytes, key: &Bytes) -> Bytes;
    fn decrypt(cipher_text: &Bytes, key: &Bytes) -> Bytes;
}