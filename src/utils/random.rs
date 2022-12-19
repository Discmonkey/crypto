use::rand;
use rand::random;
use crate::bitstring::bytes::Bytes;

pub fn flip_coin() -> bool {
   rand::random()
}

pub fn gen_random_bytes(length: usize) -> Bytes {
    let mut random_bytes = Bytes::new_zeros(length);
    for i in 0..length {
        random_bytes.bytes[i] = rand::random::<u8>();
    }

    random_bytes
}

pub fn random_byte_between(min: u8, max: u8) -> u8 {
    return rand::
}


#[cfg(test)]
mod test {
    use crate::utils::random::{flip_coin, gen_random_bytes};

    #[test]
    pub fn rest_random() {
        /// this test should fail twice out of every 2^100 runs.
        let mut found_true = false;
        let mut found_false = false;
        for _ in 0..100 {
            if flip_coin() {
                found_true = true;
            } else {
                found_false = true;
            }
        }

        assert!(found_false & found_true);
    }

    #[test]
    pub fn test_gen_random_key() {
        /// this test should fail
        let key1 = gen_random_bytes(3).to_utf8();
        let key2 = gen_random_bytes(3).to_utf8();
        let key3 = gen_random_bytes(3).to_utf8();

        assert_ne!(key1, key2);
        assert_ne!(key1, key3);
        assert_ne!(key2, key3);
    }
}