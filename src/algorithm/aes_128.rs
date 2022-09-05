use std::io::Read;
use std::convert::TryInto;
use aes::{Aes128 as Aes128Backend, Block};
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};
use crate::algorithm::Algorithm;
use crate::bitstring::bytes::Bytes;
use crate::utils::split_bytes::split_into_n;


pub struct Aes128 {
}

fn vec_to_array(v: Vec<u8>) -> [u8; 16] {
    v.try_into().unwrap()
}

fn to_blocks(bytes: &Bytes) -> Vec<Block> {
    split_into_n(bytes, bytes.len() / 128).into_iter().map(|b| {
        Block::from(vec_to_array(b.bytes))
    }).collect()
}

fn from_blocks(blocks: Vec<Block>) -> Bytes {
    let mut ret = Bytes::new();
    blocks.into_iter().for_each(|b| {
        b.into_iter().for_each(|byte| {
            ret.push_byte(byte)
        })
    });

    ret
}

impl Algorithm for Aes128 {
    fn encrypt(&self, message: &Bytes, key: &Bytes) -> Bytes {
        unimplemented!()
    }

    fn decrypt(&self, cipher_text: &Bytes, key: &Bytes) -> Bytes {
        assert_eq!(key.bytes.len(), 16);
        assert_eq!(cipher_text.bytes.len() % 16, 0);

        let key_aes = vec_to_array(key.bytes.clone());
        let cipher = Aes128Backend::new(&Block::from(key_aes));
        let mut blocks = to_blocks(cipher_text);
        cipher.decrypt_blocks(&mut blocks);

        from_blocks(blocks)
    }
}