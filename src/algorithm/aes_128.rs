use std::io::Read;
use std::convert::TryInto;
use aes::{Aes128 as Aes128Backend, Block};
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};
use crate::algorithm::Algorithm;
use crate::bitstring::bytes::Bytes;


pub struct Aes128 {
}

fn vec_to_array(v: Vec<u8>) -> [u8; 16] {
    v.try_into().unwrap()
}

fn to_blocks(bytes: &Bytes) -> Vec<Block> {
    bytes.to_blocks(16).into_iter().map(|b| {
        Block::from(vec_to_array(b.bytes))
    }).collect()
}

fn from_blocks(blocks: Vec<Block>) -> Bytes {
    let mut ret = Bytes::new();
    blocks.into_iter().for_each(|b| {
        b.iter().for_each(|byte| {
            ret.push_byte(*byte)
        })
    });

    ret
}

impl Algorithm for Aes128 {
    fn encrypt(&self, message: &Bytes, key: &Bytes) -> Bytes {
        let key_aes = vec_to_array(key.bytes.clone());
        let cipher = Aes128Backend::new(&Block::from(key_aes));
        let mut blocks = to_blocks(message);
        cipher.encrypt_blocks(&mut blocks);

        from_blocks(blocks)
    }

    fn decrypt(&self, cipher_text: &Bytes, key: &Bytes) -> Bytes {
        assert_eq!(key.bytes.len(), 16, "only support key length 16");

        let key_aes = vec_to_array(key.bytes.clone());
        let cipher = Aes128Backend::new(&Block::from(key_aes));
        let mut blocks = to_blocks(cipher_text);
        cipher.decrypt_blocks(&mut blocks);

        from_blocks(blocks)
    }
}

#[cfg(test)]
mod test {
    use crate::algorithm::aes_128::Aes128;
    use crate::algorithm::Algorithm;
    use crate::bitstring::bytes::Bytes;

    #[test]
    fn test_true() {
        let alg = Aes128{};
        let mut message = Bytes::from_utf8("This is a test");
        message.pad_pkcs(16);
        let mut key = Bytes::from_utf8("aleah");
        key.pad_pkcs(16);
        let cipher = alg.encrypt(&message, &key);
        let decoded = alg.decrypt(&cipher, &key);

        let left = message.to_utf8();
        let right = decoded.to_utf8();

        assert_eq!(left, right)
    }
}