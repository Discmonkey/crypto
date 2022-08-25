use std::cmp::Ordering;
use std::collections::binary_heap;
use std::io::Read;

use crate::algorithm::Algorithm;
use crate::algorithm::single_byte_xor::SingleByteXor;
use crate::bitstring::bytes::Bytes;
use crate::distribution::Distribution;
use crate::key_finder::{KeyFinder, KeyGenerator};
use crate::key_finder::scored_key::ScoredKey;
use crate::utils;

pub struct ByteXorKeyFinder {
    ground_truth: Distribution,
}


impl ByteXorKeyFinder {
    pub fn new() -> Option<Self> {
        let mut d = Distribution::new(true);
        let corpus = utils::readfile::read_ut8("test/corpus")?;

        d.load_and_count(&corpus);

        Some(ByteXorKeyFinder {
            ground_truth: d
        })
    }
}

impl KeyFinder for ByteXorKeyFinder {
    fn find_key(&self, num_keys: usize, cipher_text: &Bytes) -> Vec<ScoredKey> {
        let mut heap = binary_heap::BinaryHeap::with_capacity(num_keys);
        let algo = SingleByteXor {};

        for key in 0..255 {
            let message = algo.decrypt(cipher_text, &Bytes::from_byte(key));
            let mut d = Distribution::new(true);
            d.load_and_count(&message);
            let score = self.ground_truth.kl_divergence_to(&d);

            heap.push(ScoredKey {
                score: -score,
                bytes: Bytes::from_byte(key),
            })
        }
        let mut ret = vec![];
        for _ in 0..num_keys {
            ret.push(heap.pop().unwrap());
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use std::collections::binary_heap;
    use crate::bitstring::bytes::Bytes;
    use crate::key_finder::byte_xor_key_finder::ScoredKey;

    #[test]
    fn sortable_heap() {
        let mut heap = binary_heap::BinaryHeap::new();
        heap.push(ScoredKey {
            bytes: Bytes::from_bitstring("100").unwrap(),
            score: -5.0
        });

        heap.push(ScoredKey {
            bytes: Bytes::from_bitstring("010").unwrap(),
            score: -4.0
        });

        heap.push(ScoredKey {
            bytes: Bytes::from_bitstring("001").unwrap(),
            score: -7.0
        });

        assert_eq!(heap.pop().unwrap().bytes.to_bitstring(), "010");
    }
}