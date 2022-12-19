use crypto::bitstring::bytes;

fn main() {
    let mut b = bytes::Bytes::from_utf8("YELLOW SUBMARINE");
    b.pad_pkcs(20);

    println!("{}", b.to_utf8());
}