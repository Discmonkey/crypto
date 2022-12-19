use crypto::algorithm::aes_128_cbc::Aes128Cbc;
use crypto::utils;
use crypto::algorithm::Algorithm;
use crypto::bitstring::bytes::Bytes;

fn main() {
    let cipher_text = utils::readfile::read_base64("test/10.txt").unwrap();
    let key = Bytes::from_utf8("YELLOW SUBMARINE");
    let algo = Aes128Cbc::new(Bytes::new_zeros(16));
    println!("{}", algo.decrypt(&cipher_text, &key).to_utf8());
}
