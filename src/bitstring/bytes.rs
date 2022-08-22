use std::fmt::{Write};
static BASE_64: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
static HEX: &str = "0123456789abcdef";
const BIT_SETTERS: &'static [u8] = &[
    0b1000_0000,
    0b0100_0000,
    0b0010_0000,
    0b0001_0000,
    0b0000_1000,
    0b0000_0100,
    0b0000_0010,
    0b0000_0001,
];

#[derive(Debug, Clone)]
pub struct Bytes {
    pub bytes: Vec<u8>,
    bit_length: usize,
}

impl Bytes {
    // constructors
    pub fn new() -> Self {
        Self {
            bytes: vec!(),
            bit_length: 0
        }
    }

    pub fn read_bit(&self, byte: usize, bit: usize) -> u8 {
        self.bytes[byte] & BIT_SETTERS[bit]
    }

    pub fn push_byte(&mut self, byte: u8) {
        self.bit_length += 8;
        let remainder = self.bit_length % 8;
        if remainder == 0 {
            self.bytes.push(byte);
        } else {
            // move the start of the next byte over by the number of bits we currently have
            // and or the result to set the open bits
            *self.bytes.last_mut().unwrap() |= byte >> remainder;
            // now push over the bits that were not previously set to the start of the next byte
            self.bytes.push(byte << (8 - remainder));
        }
    }

    pub fn push_bit(&mut self, bit: u8) {
        if self.bit_length % 8 == 0 {
             self.bytes.push(if bit > 0 { BIT_SETTERS[0] } else { 0 });
        } else if bit > 0 {
             *self.bytes.last_mut().unwrap() += BIT_SETTERS[self.bit_length % 8];
        }
        self.bit_length += 1;
    }

    /// read n bits is meant for reads smaller than byte, otherwise just use the regular byte api
    pub fn read_n_bits(&self, at: usize, n: usize) -> u8 {
        assert!(n <= 8);
        let mut byte_idx = at / 8;
        let mut bit_idx = at % 8;
        let mut val = 0;

        for setter in BIT_SETTERS.iter().skip(8 - n) {
            if self.read_bit(byte_idx, bit_idx) > 0 {
                val |= setter
            }
            bit_idx += 1;
            if bit_idx == 8 {
                bit_idx = 0;
                byte_idx += 1;
            }
        }

        val
    }

    /// reads in bytes encoded as hex, returns a Bytes object when decoding is successful
    /// converts hex to Bytes by reading two characters at at time.
    pub fn from_hex(string: &str) -> Option<Self> {
        // handle zero padding later?
        let mut bytes = vec![];
        let mut bit_length = 0;
        for char in string.chars() {
            if let Some(int_val) = hex_to_u8(char) {
                if bit_length % 8 == 4 {
                    *bytes.last_mut().unwrap() += int_val;
                } else {
                    bytes.push(int_val * 16)
                }
                bit_length += 4
            } else {
                return None
            }
        }

        Some(Self { bytes, bit_length })
    }

    pub fn to_hex(&self) -> String {
        let mut out = String::new();

        for byte in &self.bytes {
            let index1 = byte >> 4;
            let index2 = byte & 15;

            out.write_char(HEX.chars().nth(index1 as usize).unwrap()).expect("welp");
            out.write_char(HEX.chars().nth(index2 as usize).unwrap()).expect("welp");
        }

        return out
    }

    pub fn from_utf8(string: &str) -> Self {
        Self {
            bytes: string.bytes().collect(),
            bit_length: string.bytes().len(),
        }
    }

    pub fn to_utf8(&self) -> String {
        self.bytes.iter().map(|&c| c as char).collect()
    }

    pub fn from_base64(string: &str) -> Option<Self> {
        let mut bytes = vec![0];
        let mut bit_length= 0;
        let mut bit_index = 0;
        for char in string.chars() {
            let value = base64_to_int(char)?;
            for setter in BIT_SETTERS.iter().skip(2) {
                if setter & value > 0 {
                    *bytes.last_mut().unwrap() += BIT_SETTERS[bit_index];
                }
                bit_index += 1;
                bit_length += 1;

                if bit_index == 8 {
                    bit_index = 0;
                    bytes.push(0);
                }
            }
        }

        Some(Self {
            bytes,
            bit_length
        })
    }
    // Writes out bytes as base 64, reads three bytes at a time and writes out 4 base 64 characters
    pub fn to_base64(&self) -> String {
        let mut out = String::new();

        for i in (0..self.bit_length).step_by(6) {
            let val = self.read_n_bits(i, 6);
            let char = BASE_64.chars().nth(val as usize).unwrap();
            out.write_char(char).unwrap();
        }

        out
    }

    pub fn from_bitstring(string: &str) -> Option<Self> {
        let mut bits = Self::new();
        for char in string.chars() {
            match char {
                '0' => bits.push_bit(0),
                '1' => bits.push_bit(1),
                _   => return None,
            }
        }

        Some(bits)
    }

    pub fn to_bitstring(&self) -> String {
        let mut s = String::new();
        for i in 0..self.bit_length {
            if self.read_bit(i / 8, i % 8) > 0 {
                s.write_char('1').unwrap();
            } else {
                s.write_char('0').unwrap();
            }
        }

        s
    }

    pub fn xor(&self, other: &Self) -> Self {
        Self {
            bytes: self.bytes.iter().zip(other.bytes.iter()).map(|(a, b)| {
                a ^ b
            }).collect(),
            bit_length: self.bit_length
        }
    }

    pub fn count_ones(&self) -> usize {
        let mut total = 0;
        for byte in self.bytes.iter() {
            for setter in BIT_SETTERS.iter() {
                if setter & byte > 0 {
                    total += 1;
                }
            }
        }
        total
    }

    pub fn len(&self) -> usize {
        return self.bit_length
    }
}

macro_rules! offset {
    ($char:tt, $by:expr) => {
        ($char as u8) - ($by as u8)
    }
}

fn hex_to_u8(h: char) -> Option<u8> {
    match h {
        '0'..='9' => {
            Some(offset!(h, '0'))
        },

        'a'..='f' => {
            Some(offset!(h, 'a') + 10)
        }

        _ => {
            None
        }
    }
}

fn base64_to_int(h: char) -> Option<u8> {
    match h {
        'A'..='Z' => Some(offset!(h, 'A')),
        'a'..='z' => Some(offset!(h, 'a') + 26),
        '0'..='9' => Some(offset!(h, '0') + 52),
        '+' => Some(62),
        '/' => Some(63),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use crate::bitstring::bytes::Bytes;

    #[test]
    fn bitstring_basics() {
        let s = Bytes::from_bitstring("0001");
        assert_eq!(s.unwrap().to_bitstring(), "0001");
    }

    #[test]
    fn from_hex() {
        let s = Bytes::from_hex("22").unwrap();
        assert_eq!(s.to_bitstring(), "00100010");
        assert_eq!(s.to_hex(), "22");
    }

    #[test]
    fn from_base64() {
        let s = Bytes::from_base64("BB").unwrap();
        assert_eq!(s.to_bitstring(), "000001000001");
        assert_eq!(s.to_base64(), "BB");
    }

    #[test]
    fn xor() {
        let a = Bytes::from_bitstring("1100").unwrap();
        let b = Bytes::from_bitstring("0101").unwrap();

        assert_eq!(a.xor(&b).to_bitstring(), "1001");
    }

    #[test]
    fn count_ones() {
        assert_eq!(Bytes::from_bitstring("11001100111").unwrap().count_ones(), 7);
    }
}