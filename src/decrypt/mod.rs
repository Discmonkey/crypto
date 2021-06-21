pub mod frequency;
pub mod score;

use std::collections::binary_heap;
use crate::buffer::Bytes;
use std::cmp::Ordering;
use crate::decrypt::score::score_string;

#[derive(Debug, Clone)]
pub struct Decryption {
   pub score: f64,
   pub message: String,
}

pub fn single_char_cipher(input: Bytes) -> Vec<Decryption> {

   let mut cipher = Bytes::new(input.len());

   let mut heap = vec!();

   for c in 0..256 {
      cipher.write_char((c as u8) as char, 0, 1);

      let out = input.xor(&cipher);
      let message = out.to_utf8();
      let score = score_string(&message);

      heap.push(Decryption{
         score, message
      });
   }

   heap.sort_by(|a, b| {
      b.score.partial_cmp(&a.score).unwrap()
   });

   heap
}

pub fn find_single_char_encryption(many_bytes: Vec<Bytes>) -> Vec<Decryption> {
   let mut heap = vec!();

   for line in many_bytes {
      let v = single_char_cipher(line);

      for i in 0..5 {
         heap.push(v[i].clone());
      }

   }

   heap.sort_by(|a, b| {
      b.score.partial_cmp(&a.score).unwrap()
   });

   heap
}

#[cfg(test)]
mod tests {
   use crate::buffer::Bytes;
   use crate::decrypt::{single_char_cipher, find_single_char_encryption};
   use crate::utils::readfile::read_as_bytes;

   #[test]
   fn decode_() {
      let bytes = Bytes::from_hex_string("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap();
      let out = single_char_cipher(bytes);

      for mes in out {
         println!("{:?}\n", mes);
      }
   }

   #[test]
   fn multi_decode() {
      let messages = read_as_bytes("src/decrypt/4.txt");
      let out = find_single_char_encryption(messages);

      for mes in out {
         println!("{:?}\n", mes);
      }
   }
}

