use crypto::algorithm::Algorithm;
use crypto::utils;
use crypto::algorithm::repeating_xor::RepeatingXor;
use crypto::bitstring::bytes::Bytes;

fn main() {
    let message = Bytes::from_utf8("Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal");
    let key = Bytes::from_utf8("ICE");
    let alg = RepeatingXor{};

    let cipher = alg.encrypt(&message, &key);

    assert_eq!(cipher.to_hex(), "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
}