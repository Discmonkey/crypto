use std::fmt::{Write, Formatter};
use std::ops::Deref;

static BASE_64: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
static HEX: &str = "0123456789abcdef";

#[derive(Debug, Clone)]
pub struct Bytes {
    pub bytes: Vec<u8>
}

pub fn debug_bytes(bytes: Bytes) {
   println!("[{}, {} ... {}]", bytes.bytes[0], bytes.bytes[1], bytes.bytes[-1])
}


pub fn hamming_distance(a: &[u8], b: &[u8]) -> f64  {
   a.iter().zip(b.iter()).map(|(a, b)| {
        let mut bit_dif = 0;
        let mut dif = a ^ b;

        for _ in 0..7 {
            bit_dif += 1 & dif;
            dif >>= 1;
        }

        f64::from(bit_dif)
    }).sum()
}


impl Bytes {

    pub fn new(size: usize) -> Self {
        Self {
            bytes: vec![0; size]
        }
    }

    /// write a pattern into the buffer
    pub fn write_char(&mut self, c: char, offset: usize, step:usize) {
        let mut index = offset;
        let end = self.bytes.len();


        while index < end {
            self.bytes[index.clone()] = (c as u8);
            index += &step;
        }
    }
    /// reads in bytes encoded as hex, returns a Bytes object when decoding is successful
    ///
    /// converts hex to Bytes by reading two characters at at time.
    pub fn from_hex_string(string: &str) -> Option<Self> {
        // handle zero padding later?

        let mut bytes: Vec<u8> = vec![0; string.len() / 2];

        let evens = string.chars().step_by(2);
        let odds = string.chars().skip(1).step_by(2);

        for (i, (h1, h2)) in evens.zip(odds).enumerate() {
            if let (Some(a), Some(b)) = (hex_to_u8(h1), hex_to_u8(h2)) {
                bytes[i] = a * 16 + b
            } else {
                return None
            }
        }

        Some(Bytes{bytes})
    }

    pub fn from_utf8_string(string: &str) -> Self {
        Self {
            bytes: string.bytes().collect()
        }
    }

    // Writes out bytes as base 64, reads three bytes at a time and writes out 4 base 64 characters
    pub fn to_base64_string(&self) -> String {
        let mut out = String::new();

        let mut remainder = 0;
        let mut remainder_bits = 0;

        for byte in &self.bytes {
            if remainder_bits == 0 {
                out.write_char(BASE_64.chars().nth((byte >> 2) as usize).unwrap());
                remainder_bits = 2;
                remainder = (byte & 1) * 16 + ((byte & 2) >> 1) * 32;
            } else if remainder_bits == 2 {
                let index = (byte >> 4) + remainder;
                out.write_char(BASE_64.chars().nth(index as usize).unwrap());
                remainder_bits = 4;
                remainder = (byte & 1) * 4 + ((byte & 2) >> 1) * 8 + ((byte & 4) >> 2) * 16 + ((byte & 8) >> 3) * 32;
            } else if remainder_bits == 4 {
                let index1 = (byte >> 6) + remainder;
                let index2 = byte & 63;

                out.write_char(BASE_64.chars().nth(index1 as usize).unwrap());
                out.write_char(BASE_64.chars().nth(index2 as usize).unwrap());
                remainder_bits = 0;
                remainder = 0;
            }
        }

        if remainder > 0 {
            out.write_char(BASE_64.chars().nth(remainder as usize).unwrap());
        }
        out
    }

    pub fn from_base_64(string: &str) -> Self {
        let mut bytes = vec!();
        let mut decoded = 0;
        let mut bits_written = 0;
        for char in string.chars() {
            let maybe_value = base64_to_int(char);

            if let Some(value) = maybe_value {

                match bits_written {
                    0 => {
                        decoded = value;
                        bits_written = 6;
                    }

                    6 => {
                        decoded = (decoded << 2) | (value >> 4);
                        bytes.push(decoded);

                        decoded = value & 15;
                        bits_written = 4;
                    }

                    4 => {
                        decoded = (decoded << 4) | (value >> 2);
                        bytes.push(decoded);

                        decoded = value & 3;
                        bits_written = 2;
                    }

                    2 => {
                        decoded = (decoded << 6) | value;
                        bytes.push(decoded);

                        decoded = 0;
                        bits_written = 0;
                    }

                    _ => ()
                }
            }
        }

        if bits_written != 0 {
            bytes.push(decoded);
        }

        Self {
            bytes
        }
    }

    pub fn to_hex(&self) -> String {
        let mut out = String::new();

        for byte in &self.bytes {
            let index1 = byte >> 4;
            let index2 = (byte & 15);

            out.write_char(HEX.chars().nth(index1 as usize).unwrap());
            out.write_char(HEX.chars().nth(index2 as usize).unwrap());
        }

        return out
    }



    pub fn xor(&self, other: &Self) -> Self {
        Self {
            bytes: self.bytes.iter().zip(other.bytes.iter()).map(|(a, b)| {
                a ^ b
            }).collect()
        }
    }


    pub fn repeating_xor(&self, other: &Self) -> Self {
        let cycle = other.bytes.iter().cycle();

        Self {
            bytes: self.bytes.iter().zip(cycle).map(|(a, b)| {
                a ^ b
            }).collect()
        }
    }

    pub fn to_utf8(&self) -> String {
        self.bytes.iter().map(|&c| c as char).collect()
    }

    pub fn len(&self) -> usize {
        return self.bytes.len()
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
            Some((h as u8) - ('a' as u8) + 10)
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
    use crate::bytes::{Bytes, hamming_distance};

    #[test]
    fn conversion_test() {
        let bytes = Bytes::from_hex_string("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();
        let out = bytes.to_base64_string();

        println!("{:?}\n", out);
        println!("{}", bytes.to_hex());
        assert_eq!(&out, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }

    #[test]
    fn xor_test() {
        let first = Bytes::from_hex_string("1c0111001f010100061a024b53535009181c").unwrap();
        let second = Bytes::from_hex_string("686974207468652062756c6c277320657965").unwrap();

        let out = first.xor(&second);

        assert_eq!(out.to_hex(), "746865206b696420646f6e277420706c6179")
    }

    #[test]
    fn base_64_read_test() {
        let first = Bytes::from_hex_string("1c0111001f010100061a024b53535009181c").unwrap();
        let second = first.to_base64_string();
        let third = Bytes::from_base_64(&second);

        assert_eq!(third.to_hex(), "1c0111001f010100061a024b53535009181c")
    }

    #[test]
    fn test_hamming_distance() {
        let a = Bytes::from_utf8_string("this is a test");
        let b = Bytes::from_utf8_string("wokka wokka!!!");

        assert_eq!(hamming_distance(&a.bytes, &b.bytes), 37.0);
    }
}

// 0 0 0 0
//
