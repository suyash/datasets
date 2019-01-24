//! text datasets.

use std::error::Error;

use crate::Dataset;

pub mod babi;
pub mod imdb;

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

/// shorthand for babi::load_en_single_supporting_fact_task
pub fn babi_en_single_supporting_fact_task() -> Result<
    (
        impl Dataset<
            Item = (
                std::string::String,
                std::string::String,
                (std::string::String, std::string::String, usize),
            ),
        >,
        impl Dataset<
            Item = (
                std::string::String,
                std::string::String,
                (std::string::String, std::string::String, usize),
            ),
        >,
    ),
    Box<dyn Error>,
> {
    babi::load_en_single_supporting_fact_task(
        &dirs::home_dir()
            .unwrap()
            .join(".datasets")
            .join("babi")
            .join("tasks"),
    )
}

/// shorthand for babi::load_hn_single_supporting_fact_task
pub fn babi_hn_single_supporting_fact_task() -> Result<
    (
        impl Dataset<
            Item = (
                std::string::String,
                std::string::String,
                (std::string::String, std::string::String, usize),
            ),
        >,
        impl Dataset<
            Item = (
                std::string::String,
                std::string::String,
                (std::string::String, std::string::String, usize),
            ),
        >,
    ),
    Box<dyn Error>,
> {
    babi::load_hn_single_supporting_fact_task(
        &dirs::home_dir()
            .unwrap()
            .join(".datasets")
            .join("babi")
            .join("tasks"),
    )
}
