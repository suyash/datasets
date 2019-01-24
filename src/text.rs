//! text datasets.

use std::error::Error;

use crate::Dataset;

pub mod babi;
pub mod imdb;
pub mod shakespeare;

/// a shorthand for imdb_reviews::load, will simply download and load from `$HOME/.datasets/mnist`
pub fn imdb_reviews() -> Result<
    (
        impl Dataset<Item = (String, u8)>,
        impl Dataset<Item = (String, u8)>,
    ),
    Box<dyn Error>,
> {
    imdb::reviews(
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

/// shorthand for shakespeare::shakespeare_100000
pub fn shakespeare_100000() -> Result<String, Box<dyn Error>> {
    shakespeare::shakespeare_100000(
        &dirs::home_dir()
            .unwrap()
            .join(".datasets")
            .join("shakespeare")
            .join("shakespeare_100000"),
    )
}
