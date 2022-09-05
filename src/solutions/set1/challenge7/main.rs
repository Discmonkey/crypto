use crypto::utils;
use crypto::algorithm::aes_128::Aes128;
use crypto::algorithm::Algorithm;
use crypto::bitstring::bytes::Bytes;

fn main() {
    let cipher_text = utils::readfile::read_base64("test/7.txt").unwrap();
    let key = Bytes::from_utf8("YELLOW SUBMARINE");
    let algo = Aes128{};

    println!("{}", algo.decrypt(&cipher_text, &key).to_utf8());
}