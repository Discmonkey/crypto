use crypto::bitstring::bytes::Bytes;
use crypto::utils::random::flip_coin;

enum WhichAlgorithm {
    ECB,
    CBC
}



pub fn encryption_oracle(input: Bytes) -> (Bytes, WhichAlgorithm) {
    let noisy_input =
    if flip_coin() {
        alg =
    } else {

    }
}


fn main() {}