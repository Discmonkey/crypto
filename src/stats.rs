use std::ops::Add;
use std::ops::Div;

pub struct Normal {
    mean: f64,
    std: f64,
}
//
// impl Normal {
//
//
//     pub fn fit<T: f64::From<T>>(samples: &[T]) {
//         let mean: f64 = samples.iter().map(|sample| sample.into()).sum() / samples.len().into();
//
//         let std = f64::sqrt(samples.iter().map(|sample| {
//             f64::powf(sample.into() - mean, 2.0)
//         })).sum();
//
//
//     }
// }

#[cfg(test)]
mod test {
    use crate::stats::Normal;

    #[test]
    fn test_fit() {
        Normal::fit(&[1, 1, 1]);
    }
}