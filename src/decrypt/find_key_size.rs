use crate::bytes::Bytes;
use crate::bytes::debug_bytes;

pub type KeySize = usize;

pub struct KeyStats {
   pub mean: f64,
   pub std: f64,
   pub size: KeySize
}


pub fn find_key_size(bytes: Bytes, start:usize, end:usize) -> Vec<KeySize> {
   vec!()
}

/// finds the average hamming distance between consecutive sub slices of bytes
fn score_key(cipher: Bytes, length: usize) -> Key {
   let normalizer = f64::from(length);
   let cipher_len = cipher.len();

   let mut ref_vec = vec![];
   for i in (0..bytes.bytes.len()).step_by(length) {
      ref_vec.push(cipher.bytes[i..usize::min(i + 4, cipher_len)])
   }

}

