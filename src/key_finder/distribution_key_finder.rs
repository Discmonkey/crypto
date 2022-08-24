use std::cmp::Ordering;
use std::collections::binary_heap;
use std::io::Read;

use crate::algorithm::Algorithm;
use crate::bitstring::bytes::Bytes;
use crate::distribution::Distribution;
use crate::key_finder::{KeyFinder, KeyGenerator};
use crate::utils;

struct DistributionKeyFinder {
    ground_truth: Distribution,
}

struct ScoredKey {
    pub score: f64,
    pub bytes: Bytes,
}


impl Ord for ScoredKey {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.score == other.score {
            Ordering::Equal
        } else if self.score < other.score {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl Eq for ScoredKey {

}

impl PartialOrd for ScoredKey {
    fn partial_cmp(&self, other: &ScoredKey) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ScoredKey {
    fn eq(&self, other: &ScoredKey) -> bool {
        self.score == other.score
    }
}


impl DistributionKeyFinder {
    fn new() -> Option<Self> {
        let mut d = Distribution::new(true);
        let corpus = utils::readfile::read_ut8("test/corpus")?;

        d.load_and_count(&corpus);

        Some(DistributionKeyFinder {
            ground_truth: d
        })
    }
}

impl KeyFinder for DistributionKeyFinder {
    fn find_key(&self, num_keys: usize, cipher_text: Bytes, algo: &dyn Algorithm, key_gen: &dyn KeyGenerator) -> Vec<Bytes> {
        let mut heap = binary_heap::BinaryHeap::with_capacity(num_keys);

        while let Some(key) = key_gen.next() {
            let message = algo.decrypt(&cipher_text, &key);
            let mut d = Distribution::new(false);
            d.load_and_count(&message);
            let score = self.ground_truth.kl_divergence_to(&d);

            heap.push(ScoredKey {
                score: -score,
                bytes: message
            })
        }

        heap.into_sorted_vec().iter().map(|k| k.bytes).collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::binary_heap;
    use crate::bitstring::bytes::Bytes;
    use crate::key_finder::distribution_key_finder::ScoredKey;

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