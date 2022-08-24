use crate::bitstring::bytes::Bytes;

pub struct Distribution {
    counts: Vec<usize>,
    odds: Vec<f64>,
}

impl Distribution {
    pub fn new(initialize_with_one: bool) -> Self {
        Distribution {
            counts: vec![if initialize_with_one { 1 } else { 0 }; 255],
            odds: vec![0.0;255]
        }
    }

    pub fn load_and_count(&mut self, bytes: &Bytes) {
        bytes.bytes.iter().for_each(|b| {
            self.counts[*b as usize] += 1;
        });
        let divisor = self.sum_counts();
        for i in 0..256 {
            self.odds[i] = f64::from(self.counts[i] as i32) / divisor
        }
    }

    fn sum_counts(&self) -> f64 {
        let mut total = 0.0;
        self.counts.iter().for_each(|c| {
            total += f64::from(*c as i32);
        });
        total
    }

    /// Computes D_kl(P || Q), where P is self, ie P is presumed to be the "true" distribution
    pub fn kl_divergence_to(&self, other: &Self) -> f64 {
        let mut divergence = 0.0;
        for i in 0..256 {
            let p = self.odds[i];
            let q = other.odds[i];
            if p > 0.0 && q > 0.0 {
               divergence += p * (p / q).ln();
            }
        }
        divergence
    }
}