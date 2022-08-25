use crate::bitstring::bytes::Bytes;

pub fn split_into_n(bytes: Bytes, n: usize) -> Vec<Bytes> {
    let length = bytes.bytes.len() / n;
    let mut ret = vec![];
    for i in 0..n {
        let mut temp_bytes = Bytes::new();
        for j in 0..length {
            temp_bytes.push_byte(bytes.bytes[i * length + j]);
        }

        ret.push(temp_bytes);
    }

    ret
}