use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::bitstring::bytes::Bytes;

pub fn read_as_bytes<P>(filename: P) -> Vec<Bytes>
    where P:AsRef<Path> {
    let mut bytes_vec: Vec<Bytes> = vec!();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(parsed) = line {
                if let Some(s) = Bytes::from_hex_string(&parsed) {
                    bytes_vec.push(s)
                }
            }
        }
    }

    bytes_vec
}

pub fn read_in_base_64<P: AsRef<Path>>(filename: P) -> Option<Bytes> {
    let mut base_64 = String::new();

    let lines = read_lines(filename).ok()?;

    for line in lines {
        if let Ok(parsed) = line {
            base_64.push_str(&parsed);
        }
    }

    Some(Bytes::from_base_64(&base_64))
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
