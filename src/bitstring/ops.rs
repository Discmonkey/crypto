use crate::bitstring::bytes::Bytes;

pub fn edit_distance(b1: &Bytes, b2: &Bytes) -> usize {
    b1.xor(b2).count_ones()
}


#[cfg(test)]
mod tests {
    use crate::bitstring::bytes::Bytes;
    use crate::bitstring::ops::edit_distance;
    #[test]
    fn test_edit_distance() {
        let l = Bytes::from_utf8("this is a test");
        let r = Bytes::from_utf8("wokka wokka!!!");

        assert_eq!(edit_distance(&l, &r), 37)
    }
}