use crate::algorithm::Algorithm;
use crate::bitstring::bytes::Bytes;

pub struct RepeatingXor{}

impl Algorithm for RepeatingXor {
    fn encrypt(&self, message: &Bytes, key: &Bytes) -> Bytes {
        Bytes::from_bytes(message.bytes.iter().zip(key.bytes.iter().cycle()).map(
            |(a, b)| {
                a ^ b
            }
        ).collect())
    }

    fn decrypt(&self, cipher_text: &Bytes, key: &Bytes) -> Bytes {
        self.encrypt(cipher_text, key)
    }
}

