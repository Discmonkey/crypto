use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::bitstring::bytes::Bytes;

pub fn read_hex<P>(filename: P) -> Vec<Bytes>
    where P:AsRef<Path> {
    let mut bytes_vec: Vec<Bytes> = vec!();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(parsed) = line {
                if let Some(s) = Bytes::from_hex(&parsed) {
                    bytes_vec.push(s)
                } else {
                    println!("failed to read in hex");
                }
            }
        }
    }

    bytes_vec
}

pub fn read_base64<P: AsRef<Path>>(filename: P) -> Option<Bytes> {
    let mut base_64 = String::new();

    let lines = read_lines(filename).ok()?;

    for line in lines {
        if let Ok(parsed) = line {
            base_64.push_str(&parsed);
        }
    }

    Bytes::from_base64(&base_64)
}

pub fn read_base64_lines<P: AsRef<Path>>(filename: P) -> Option<Vec<Bytes>> {
    let mut ret = vec![];

    let lines = read_lines(filename).ok()?;

    for line in lines {
        if let Ok(parsed) = line {
            ret.push(Bytes::from_base64(&parsed)?);
        }
    }
    Some(ret)
}

pub fn read_ut8<P: AsRef<Path>>(filename: P) -> Option<Bytes> {
    let mut bytes = Bytes::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(parsed) = line {
                for char in parsed.bytes() {
                    bytes.push_byte(char);
                }
            }
        }
    }

    Some(bytes)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
