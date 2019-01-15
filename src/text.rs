//! text datasets.

use std::error::Error;

use crate::Dataset;

pub mod imdb_reviews;

/// a shorthand for imdb_reviews::load, will simply download and load from `$HOME/.datasets/mnist`
pub fn imdb_reviews() -> Result<
    (
        impl Dataset<Item = (String, u8)>,
        impl Dataset<Item = (String, u8)>,
    ),
    Box<dyn Error>,
> {
    imdb_reviews::load(
        &dirs::home_dir()
            .unwrap()
            .join(".datasets")
            .join("imdb_reviews"),
    )
}
