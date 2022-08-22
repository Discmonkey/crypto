use crate::distribution::Distribution;
use crate::key_finder::KeyFinder;
use crate::utils;

struct DistributionKeyFinder {
    ground_truth: Distribution,
}

impl DistributionKeyFinder {
    fn new() -> Self {
        let d = Distribution::new(true);
        let corpus = utils::readfile::r
        d.load_and_count()
    }
}

impl KeyFinder for DistributionKeyFinder