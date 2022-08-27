use crypto::algorithm::Algorithm;
use crypto::algorithm::repeating_xor::RepeatingXor;
use crypto::bitstring::bytes::Bytes;
use crypto::key_finder::KeyFinder;
use crypto::key_finder::repeating_key_xor::RepeatingKeyXor;
use crypto::utils;

fn main() {
    let cipher_text = utils::readfile::read_base64("test/6.txt").unwrap();
    let algo = RepeatingXor{};
    let key_finder = RepeatingKeyXor{};

    let keys = key_finder.find_key(10, &cipher_text);

    for key in keys {
        println!("{} - {} - {}", &key.score, algo.decrypt(&cipher_text, &key.bytes).to_utf8(), &key.bytes.to_utf8());
    }
}
