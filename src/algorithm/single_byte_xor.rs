use crate::algorithm::Algorithm;
use crate::bitstring::bytes::Bytes;

pub struct SingleByteXor {}

impl SingleByteXor {
    pub fn apply_xor(message: &Bytes, byte: u8) -> Bytes {
        let mut bytes = Bytes::new();
        message.bytes.iter().for_each(|b| {
            bytes.push_byte(b ^ byte)
        });

        bytes
    }
}

impl Algorithm for SingleByteXor {
    fn encrypt(&self, message: &Bytes, key: &Bytes) -> Bytes {
        SingleByteXor::apply_xor(message, key.bytes[0])
    }

    fn decrypt(&self, cipher_text: &Bytes, key: &Bytes) -> Bytes {
        SingleByteXor::apply_xor(cipher_text, key.bytes[0])
    }
}