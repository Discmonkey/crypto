pub mod key_generators;
pub mod distribution_key_finder;

use crate::algorithm::Algorithm;
use crate::bitstring::bytes::Bytes;

/// Object that generates possible keys
trait KeyGenerator {
    /// Iterates through possible keys before returning None.
    fn next(&mut self) -> Option<Bytes>;
}

/// Takes Encryption Algorithm and Key generator and returns num_keys in descending order of
/// likelihood.
trait KeyFinder {
    /// Ransk keys produced by KeyGenerator
    fn find_key(&self, num_keys: usize, cipher_text: &Bytes, algo: &dyn Algorithm)
        -> Vec<Bytes>;
}

