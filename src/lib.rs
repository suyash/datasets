#![deny(missing_docs, missing_debug_implementations)]

//! # datasets

pub mod utils;

/// A Dataset is basically an iterator, with some additional capabilities.
///
/// - `shuffle(buffer_size)`: eagerly takes buffer_size items and returns shuffled
/// - `batch(batch_size)`: an array of batch_size at a time instead of 1 at a time
pub trait Dataset: Iterator {
    /// shuffle
    fn shuffle(self, buffer_size: usize) -> Shuffle<Self>
    where
        Self: Sized,
    {
        Shuffle {
            iter: self,
            buffer_size,
        }
    }

    /// batch
    fn batch(self, batch_size: usize) -> Batch<Self>
    where
        Self: Sized,
    {
        Batch {
            iter: self,
            batch_size,
        }
    }
}

/// Shuffle
#[derive(Debug)]
pub struct Shuffle<I> {
    iter: I,
    buffer_size: usize,
}

/// Batch
#[derive(Debug)]
pub struct Batch<I> {
    iter: I,
    batch_size: usize,
}

// TODO: reconsider this, do we want all iterators be datasets.
impl<I: Iterator> Dataset for I {}
