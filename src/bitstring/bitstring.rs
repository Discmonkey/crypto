use std::fmt;
use std::ops::BitXor;
use std::ops::Index;
use fmt::Display;
use std::fmt::Formatter;

use crate::bitstring::bytes::Bytes;

type Bit = u8;

pub struct Bitstring {
    bytes: Bytes,
    zero: Bit,
    one: Bit,
}

impl Bitstring {
    fn new() -> Self {
        return Bitstring {
            bytes: Bytes { bytes: vec![] },
            zero: Bit,
            one: Bit,
        }
    }
}

impl BitXor for Bitstring {
    type Output = Bitstring;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let new = self.bytes.
    }
}

impl Index<usize> for Bitstring {
    type Output = Bit;

    fn index(&self, index: usize) -> &Self::Output {
        let byte_index= index / 4;
        let remainder = index % 4;
        return if (self.bytes.bytes[byte_index] << remainder) & 1 {
            &self.one
        } else {
            &self.zero
        }
    }
}

impl fmt::Display for Bitstring {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {

    }
}

