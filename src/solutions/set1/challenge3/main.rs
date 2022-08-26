use crypto::algorithm::Algorithm;
use crypto::bitstring::bytes::Bytes;
use crypto::algorithm::single_byte_xor::SingleByteXor;
use crypto::key_finder::byte_key_xor::ByteKeyXor;
use crypto::key_finder::KeyFinder;

fn main() {
    let cipher_text = Bytes::from_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap();
    let key_finder = ByteKeyXor::new().unwrap();
    let algo = SingleByteXor {};
    let keys = key_finder.find_key(2, &cipher_text);

    for key in keys {
        println!("{} - {}", &key.score, algo.decrypt(&cipher_text, &key.bytes).to_utf8());
    }
}