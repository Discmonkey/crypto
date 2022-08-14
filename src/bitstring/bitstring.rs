use std::fmt;
use std::ops::BitXor;
use std::ops::Index;
use fmt::Display;
use std::fmt::{Formatter, Write};

use crate::bitstring::bytes::Bytes;

type Bit = u8;

pub struct Bitstring {
    bytes: Bytes,
    length: usize,
    zero: Bit,
    one: Bit,
}

impl Bitstring {
    fn new() -> Self {
        return Bitstring {
            bytes: Bytes { bytes: vec![] },
            length: 0,
            zero: 0,
            one: 1
        }
    }

    fn set_bit(&mut self, index: usize, bit: Bit) {
        let mut mask: u8 = 1 << (index % 8);

        if bit == 1 {
            self.bytes.bytes[index / 8] |= mask;
        } else {
            self.bytes.bytes[index / 8] &= !mask;
        }
    }

    fn push_bit(&mut self, bit: Bit) {
        if self.length % 8 == 0 {
            match bit {
                0 => self.bytes.push(0),
                1 => self.bytes.push(0b1000_0000),
                _ => {}
            }
        } else {
           self.set_bit(self.length, bit);
        }

        self.length += 1;
    }

    fn push_byte(&mut self, byte: u8) {
        if self.length % 8 == 0 {
            self.bytes.bytes.push(byte);
        } else {
            let set_bits = self.length % 8;
            let open_bits= 8 - set_bits;
            // write the first 8 - extra_length bits into the old byte
            let &mut mut last = self.bytes.last();
            last |= (byte >> set_bits);
            self.bytes.push(0 | (byte << open_bits));
        }
    }

    /// reads a string such as 100011 into bits
    fn from_string(bit_pattern: &str) -> Option<Self> {
        let mut bits = Self::new();
        for c in bit_pattern.chars() {
            match c {
                '0' => bits.push_bit(0),
                '1' => bits.push_bit(1),
                _ => return None
            }
        }

        Some(bits)
    }
}

impl BitXor for Bitstring {
    type Output = Bitstring;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut new = Self::new();
        new.bytes = self.bytes.xor(&rhs.bytes);

        new
    }

}

impl Index<usize> for Bitstring {
    type Output = Bit;

    fn index(&self, index: usize) -> &Self::Output {
        let byte_index= index / 8;
        let remainder = index % 8;
        return if (self.bytes.bytes[byte_index] << remainder) & 1 == 1 {
            &self.one
        } else {
            &self.zero
        }
    }
}

impl fmt::Display for Bitstring {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for i in 0..self.length {
            write!(f, "{}", self[i])?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::bitstring::bitstring::Bitstring;

    #[test]
    fn test_read_from_str() {
        let b = Bitstring::from_string("0001").unwrap();
        assert_eq!(b.to_string(), "0001")
    }
}