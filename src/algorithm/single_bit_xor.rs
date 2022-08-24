use crate::algorithm::Algorithm;
use crate::bitstring::bytes::Bytes;

pub struct SingleBitXor {}

impl SingleBitXor {
    pub fn apply_xor(message: &Bytes, byte: u8) -> Bytes {
        let mut bytes = Bytes::new();
        message.bytes.iter().for_each(|b| {
            bytes.push_byte(b ^ byte)
        });

        bytes
    }
}

impl Algorithm for SingleBitXor {
    fn encrypt(&self, message: &Bytes, key: &Bytes) -> Bytes {
        SingleBitXor::apply_xor(message, key.bytes[0])
    }

    fn decrypt(&self, cipher_text: &Bytes, key: &Bytes) -> Bytes {
        SingleBitXor::apply_xor(cipher_text, key.bytes[0])
    }
}