pub mod byte_xor_key_finder;
pub mod scored_key;

use crate::algorithm::Algorithm;
use crate::bitstring::bytes::Bytes;
use crate::key_finder::scored_key::ScoredKey;

/// Object that generates possible keys
trait KeyGenerator {
    /// Iterates through possible keys before returning None.
    fn next(&mut self) -> Option<Bytes>;
}

/// Takes Encryption Algorithm and Key generator and returns num_keys in descending order of
/// likelihood.
pub trait KeyFinder {
    /// Ransk keys produced by KeyGenerator
    fn find_key(&self, num_keys: usize, cipher_text: &Bytes)
        -> Vec<ScoredKey>;
}

