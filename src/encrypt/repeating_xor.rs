use crate::bytes::Bytes;

pub fn repeating_xor(message: &Bytes, key: &Bytes) -> Bytes {
    let cycle = key.bytes.iter().cycle();

    Bytes {
        bytes: message.bytes.iter().zip(cycle).map(|(a, b)| {
            a ^ b
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::bytes::Bytes;
    use crate::decrypt::{single_char_cipher, find_single_char_encryption};
    use crate::utils::readfile::read_as_bytes;
    use crate::encrypt::repeating_xor::repeating_xor;

    #[test]
    fn decode_() {
        let message = Bytes::from_utf8_string("Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal");
        let key = Bytes::from_utf8_string("ICE");

        let cipher = repeating_xor(&message, &key);
        println!("{}", cipher.to_hex());
        assert_eq!(cipher.to_hex(), "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f")
    }
}