use crypto::algorithm::aes_128::Aes128;
use crypto::bitstring::bytes::Bytes;
use crypto::bitstring::ops::edit_distance;
use crypto::utils::readfile::read_base64_lines;
use std::collections::{binary_heap, BinaryHeap};
use crypto::algorithm::Algorithm;
use crypto::key_finder::scored_key::ScoredKey;

/// Count how often the same series of bytes appears
fn count_dupes(bytes: &Bytes) -> f64 {
    let blocks = bytes.to_blocks(16);
    let mut count = 0.0;
    for i in 0..blocks.len() {
        for j in 0..blocks.len() {
            if edit_distance(&blocks[i], &blocks[j]) == 0 {
                count += 1.0
            }
        }
    }
    count
}
fn main() {
    let codes = read_base64_lines("test/8.txt").unwrap();
    let mut heap = BinaryHeap::new();

    for code in codes {
        let score = count_dupes(&code);
        heap.push(ScoredKey {
            score,
            bytes: code.clone()
        })
    }

    for _ in 0..10 {
        let s = heap.pop().unwrap();
        println!("{}", s.score);
    }
}