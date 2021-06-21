
/// Returns a frequency score, for derivation
pub fn default_distribution() -> Vec<f64> {
    vec![0.0812,
         0.0149,
         0.0271,
         0.0432,
         0.1202,
         0.0230,
         0.0203,
         0.0592,
         0.0731,
         0.0010,
         0.0069,
         0.0398,
         0.0261,
         0.0695,
         0.0768,
         0.0182,
         0.0011,
         0.0602,
         0.0628,
         0.0910,
         0.0288,
         0.0111,
         0.0209,
         0.0017,
         0.0211,
         0.0007,
         0.0001]
}

pub fn char_to_index(c: char) -> Option<usize> {

    let upper = c.to_ascii_uppercase();
    match upper {
        'A'..='Z' => Some(((upper as usize) - ('A' as usize))),
        _ => Some(26)
    }
}

pub fn zero_distribution() -> Vec<f64> {
    vec![0.0;27]
}
