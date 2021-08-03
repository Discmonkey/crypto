use std::ops::Index;

// 0 (false) or 1 (true)
pub type Bit = bool;

pub struct Bits {
    ints: vec<u8>,
}

pub enum Encoding {
    Hex,
    Base64,
}

impl Bits {
    pub fn new() -> Self {
        return Self {
            ints: vec!(),
        }
    }

    pub fn decode(string: &str, encoding: Encoding) -> Self {
        Self::new()
    }

    pub fn encode(encoding: Encoding) -> Self {
        Self::new()
    }
}

impl Index<std::slice> for Bits {
    type Output = Self;

    fn index(&self, index: std::slice) -> &Self::Output {
        todo!()
    }
}