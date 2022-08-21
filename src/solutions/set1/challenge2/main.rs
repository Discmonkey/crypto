use crypto::bitstring::bytes::Bytes;

fn main() {
    let a = Bytes::from_hex("1c0111001f010100061a024b53535009181c").unwrap();
    let b = Bytes::from_hex("686974207468652062756c6c277320657965").unwrap();

    let result = a.xor(&b).to_hex();
    assert_eq!(result, "746865206b696420646f6e277420706c6179")
}