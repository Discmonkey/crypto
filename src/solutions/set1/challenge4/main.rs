use crypto::algorithm::Algorithm;
use crypto::algorithm::single_byte_xor::SingleByteXor;
use crypto::bitstring::bytes::Bytes;
use crypto::key_finder::byte_key_xor::ByteKeyXor;
use crypto::key_finder::KeyFinder;
use crypto::utils::readfile::{read_base64, read_hex};
use crypto::utils::split_bytes::split_into_n;

use std::collections::binary_heap::BinaryHeap;
use crypto::key_finder::scored_key::ScoredKey;

fn main() {
    let ciphers= read_hex("test/4.txt");

    let key_finder = ByteKeyXor::new().unwrap();
    let xor = SingleByteXor{};
    let mut best: BinaryHeap<ScoredKey> = BinaryHeap::new();

    for cipher in ciphers {
        let keys = key_finder.find_key(10, &cipher);

        keys.iter().map(|k| {
            ScoredKey {
                bytes: xor.decrypt(&cipher, &k.bytes),
                score: k.score
            }
        }).for_each(|k| {
            best.push(k);
        });
    }

    for _ in 0..50 {
        let message = best.pop().unwrap();
        println!("{} - {}", message.score, message.bytes.to_utf8());
    }

}