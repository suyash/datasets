#![deny(missing_docs, missing_debug_implementations)]

//! # datasets

mod dataset;

pub use crate::dataset::Dataset;

pub mod image;
pub mod text;

pub mod utils;
