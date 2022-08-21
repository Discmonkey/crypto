use crate::algorithm::Algorithm;
use crate::bitstring::bytes::Bytes;

struct SingleBitXor {}

impl SingleBitXor {
    fn apply_xor(message: &Bytes, byte: &Bytes) {

    }
}

impl Algorithm for SingleBitXor {
    fn encrypt(message: &Bytes, key: &Bytes) -> Bytes {
        todo!()
    }

    fn decrypt(cipher_text: &Bytes, key: &Bytes) -> Bytes {
        todo!()
    }
}