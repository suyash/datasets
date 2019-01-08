#![deny(missing_docs, missing_debug_implementations)]

//! # datasets

pub mod utils;

/// A Dataset is basically an iterator, with some additional capabilities.
/// 
/// - `shuffle(buffer_size)`: eagerly takes buffer_size items and returns shuffled
/// - `batch(batch_size)`: an array of batch_size at a time instead of 1 at a time
trait Dataset: Iterator {}
