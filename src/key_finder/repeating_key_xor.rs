use std::collections::binary_heap::BinaryHeap;
use crate::bitstring::bytes::Bytes;
use crate::bitstring::ops::edit_distance;
use crate::key_finder::byte_key_xor::ByteKeyXor;
use crate::key_finder::KeyFinder;
use crate::key_finder::scored_key::ScoredKey;

pub struct RepeatingKeyXor {

}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct ScoredLength {
    pub score: usize,
    pub length: usize,
}


fn take_first_n_of_size(bytes: &Bytes, n: usize, l: usize) -> Vec<Bytes> {
    let mut ret = vec![];

    for i in 0..n {
        let mut t_bytes = vec![];
        for j in 0..l {
            t_bytes.push(bytes.bytes[i * l + j])
        }
        ret.push(Bytes::from_bytes(t_bytes));
    }

    ret
}

fn rank_lengths(bytes: &Bytes, min: usize, max:usize) -> Vec<ScoredLength> {
    let mut stack = BinaryHeap::new();
    for l in min..=max {
        let comps = take_first_n_of_size(bytes, 4, l);
        let mut score = 0;
        for i in 0..comps.len() {
            for j in 0..comps.len() {
                score += edit_distance(&comps[i], &comps[j]);
            }
        }
        stack.push(ScoredLength {
            score: score / l,
            length: l
        })
    }
    stack.into_sorted_vec()
}

fn split(bytes: &Bytes, len: usize) -> Vec<Bytes> {
    let mut ret = vec![Bytes::new();len];
    for i in 0..bytes.bytes.len() {
        ret[i % len].push_byte(bytes.bytes[i]);
    }

    while ret[len - 1].bytes.len() < ret[len - 2].bytes.len() {
        ret[len -1].push_byte(0);
    }

    ret
}

impl KeyFinder for RepeatingKeyXor {
    fn find_key(&self, num_keys: usize, cipher_text: &Bytes) -> Vec<ScoredKey> {
        let lengths = rank_lengths(cipher_text, 2, 40);
        let byte_xor = ByteKeyXor::new().unwrap();
        let mut heap = BinaryHeap::new();
        for l in lengths.into_iter().take(5) {
            let texts =  split(cipher_text, l.length);
            let mut total_score = 0.0;
            let mut key = Bytes::new();

            for text in texts {
                let byte_keys = byte_xor.find_key(1, &text);
                total_score += byte_keys[0].score;
                key.push_byte(byte_keys[0].bytes.bytes[0]);
            }

            heap.push(ScoredKey {
                score: total_score,
                bytes: key.clone()
            })
        }

        heap.into_sorted_vec().into_iter().take(num_keys).collect()
        // find several key length candidates
    }
}