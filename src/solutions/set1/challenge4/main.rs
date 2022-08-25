use crypto::algorithm::single_byte_xor::SingleByteXor;
use crypto::key_finder::byte_xor_key_finder::ByteXorKeyFinder;
use crypto::utils::readfile::read_base64;
use crypto::utils::split_bytes::split_into_n;

fn main() {
    let b64 = read_base64("test/4.txt").unwrap();
    let num_lines = b64.bytes.len() / 60;
    let ciphers = split_into_n(b64, num_lines);

    ByteXorKeyFinder
    SingleBitXor
}