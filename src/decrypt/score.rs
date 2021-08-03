use crate::bytes;
use crate::decrypt::frequency::{zero_distribution, default_distribution, char_to_index};

/// returns a score (where lower is better) of how likely we are to see
/// the distribution of characters within a string given the natural distribution of
/// characters in the english language. For now uses the KL divergence between
pub fn score_string(string: &str) -> f64 {
    let mut workspace = zero_distribution();
    let target_dist = default_distribution();
    let mut divisor = 0.0;

    for char in string.chars() {
        if let Some(idx) = char_to_index(char) {
            workspace[idx] += 1.0;
            divisor += 1.0;
        }
    }

    let mut score = 0.0;
    let mut scratch = 0.0 as f64;

    for i in 0..workspace.len() {
        // normalize
        scratch = workspace[i] / divisor;

        // avoid zero values
        scratch += 0.00001;

        // divide by the "true value
        scratch /= target_dist[i];

        // take the log
        scratch = f64::log2(scratch);

        // then the expectation over the log
        scratch *= target_dist[i];

        score += scratch;

    }

    score
}
