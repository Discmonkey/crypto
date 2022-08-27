pub mod byte_key_xor;
pub mod scored_key;
pub mod repeating_key_xor;

use crate::bitstring::bytes::Bytes;
use crate::key_finder::scored_key::ScoredKey;

/// Finds <num_keys> that may potentially break the <cipher_text>.
pub trait KeyFinder {
    /// Ransk keys produced by KeyGenerator
    fn find_key(&self, num_keys: usize, cipher_text: &Bytes)
        -> Vec<ScoredKey>;
}

