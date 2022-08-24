use crate::bitstring::bytes::Bytes;
use crate::key_finder::KeyGenerator;

struct SingleByte {
    byte: u8,
    done: bool,
}

impl SingleByte {
    pub fn new() -> Self {
        SingleByte {
            byte: 0,
            done: false
        }
    }
}

impl KeyGenerator for SingleByte {
    fn next(&mut self) -> Option<Bytes> {
        let ret = self.byte;
        if ret == 255
        }
    }
}