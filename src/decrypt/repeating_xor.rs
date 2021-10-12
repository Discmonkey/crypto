use crate::bytes::Bytes;
use crate::bytes::debug_bytes;


pub fn find_key_size(bytes: Bytes, start:usize, end:usize) -> Vec<usize> {
   vec!()
}

fn shard_bytes(bytes: Bytes, key_size: usize) -> Vec<[u8]> {
   let mut ret = vec![];
   for i in 0..(bytes.bytes.len() / key_size) {
      let start = i * key_size;
      let end = (i + 1) * key_size;
      ret.push(bytes.bytes[start..end])
   }

   ret
}

fn score_key(cipher: Bytes, key_size: usize) -> f64 {
   let normalizer = f64::from(key_size);
   let cipher_len = cipher.len();

   0.0
}

#[cfg(test)]
mod tests {
   use crate::bytes::Bytes;
   use crate::encrypt::repeating_xor::repeating_xor;

   #[test]
   fn decrypt_repeating_xor() {
      let message = Bytes::from_utf8_string("an encrypted messsage is very nice");
      let key = "max";

      repeating_xor()
   }
}

