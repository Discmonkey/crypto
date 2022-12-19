use std::ptr::hash;
use crate::algorithm::aes_128::Aes128;
use crate::algorithm::Algorithm;
use crate::bitstring::bytes::Bytes;

pub struct Aes128Cbc {
   initialization_vector: Bytes
}

impl Aes128Cbc {
    pub fn new(initialization_vector: Bytes) -> Self {
        Self {
            initialization_vector
        }
    }

    pub fn set_initialization(&mut self, initialization_vector: Bytes) {
        self.initialization_vector = initialization_vector;
    }
}

fn from_blocks(blocks: Vec<Bytes>) -> Bytes {
    let mut bytes = Bytes::new();

    for block in blocks {
        for byte in block.bytes {
            bytes.push_byte(byte);
        }
    }

    bytes
}

impl Algorithm for Aes128Cbc {
    fn encrypt(&self, message: &Bytes, key: &Bytes) -> Bytes {
        let alg = Aes128{};
        let mut bytes_output = Bytes::new();
        let mut previous_block = self.initialization_vector.clone();

        for block in message.to_blocks(16) {
            let output_block = alg.encrypt(&block.xor(&previous_block), &key);
            previous_block = output_block.clone();
            for byte in output_block.bytes {
                bytes_output.push_byte(byte);
            }
        }

        bytes_output
    }

    fn decrypt(&self, cipher_text: &Bytes, key: &Bytes) -> Bytes {
        let alg = Aes128{};
        let mut bytes_output = Bytes::new();
        let mut previous_block = self.initialization_vector.clone();

        for block in cipher_text.to_blocks(16) {
            let hashed_output_block = alg.decrypt(&block, &key);
            let output_block = hashed_output_block.xor(&previous_block);
            previous_block = block;
            for byte in output_block.bytes {
                bytes_output.push_byte(byte);
            }
        }

        bytes_output
    }
}