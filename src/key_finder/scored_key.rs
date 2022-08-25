use std::cmp::Ordering;
use crate::bitstring::bytes::Bytes;

pub struct ScoredKey {
    pub score: f64,
    pub bytes: Bytes,
}


impl Ord for ScoredKey {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.score == other.score {
            Ordering::Equal
        } else if self.score < other.score {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl Eq for ScoredKey {

}

impl PartialOrd for ScoredKey {
    fn partial_cmp(&self, other: &ScoredKey) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ScoredKey {
    fn eq(&self, other: &ScoredKey) -> bool {
        self.score == other.score
    }
}
