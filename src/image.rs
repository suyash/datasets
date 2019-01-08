//! Image Datasets.

use std::error::Error;

use crate::Dataset;

pub mod mnist;

/// a shorthand for mnist::load, will simply download and load from `$HOME/.datasets/mnist`
pub fn mnist() -> Result<
    (
        impl Dataset<Item = (Vec<Vec<u8>>, u8)>,
        impl Dataset<Item = (Vec<Vec<u8>>, u8)>,
    ),
    Box<dyn Error>,
> {
    mnist::load(&dirs::home_dir().unwrap().join(".datasets").join("mnist"))
}
