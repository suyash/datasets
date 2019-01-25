use std::fmt;

use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

/// A Dataset is basically an iterator, with some additional capabilities.
///
/// - `shuffle(buffer_size, seed)`: eagerly takes buffer_size items and returns shuffled
/// - `batch(batch_size, drop_remainder)`: an array of batch_size at a time instead of 1 at a time
///
/// TODO:
///
/// - `padded_batch(batch_size, padding_value)`: make the dataset uniform by filling with `padding_value`.
/// - `window`?: described at https://github.com/tensorflow/community/blob/master/rfcs/20180726-tf-data-windowing-reducers.md
///
/// The goal is for this interface to be at feature parity with `tensorflow.data.Dataset`.
pub trait Dataset: Iterator {
    /// shuffle
    /// TODO: handle error when batch_size is 0
    fn shuffle(self, buffer_size: usize, seed: u64) -> Shuffle<Self>
    where
        Self: Sized,
    {
        Shuffle::new(self, buffer_size, seed)
    }

    /// batch
    /// TODO: handle error when batch_size is 0
    fn batch(self, batch_size: usize, drop_remainder: bool) -> Batch<Self>
    where
        Self: Sized,
    {
        Batch {
            iter: self,
            batch_size,
            drop_remainder,
        }
    }
}

// TODO: reconsider this, do we want all iterators be datasets.
impl<I> Dataset for I where I: Iterator {}

/// Shuffle is an iterator that returns the elements of the inner iterator in a shuffled order.
///
/// ```
/// use datasets::Dataset;
///
/// let v: Vec<usize> = (0..8).shuffle(5, 0).collect();
/// assert_eq!(v.len(), 8);
/// assert_eq!(v, vec![4, 2, 0, 3, 7, 6, 5, 1]);
/// ```
///
/// TODO: implement `reshuffle_each_iteration` as defined at https://www.tensorflow.org/api_docs/python/tf/data/Dataset#shuffle.
pub struct Shuffle<I>
where
    I: Iterator,
{
    iter: I,
    buffer_size: usize,
    buffer: Vec<Option<<I as Iterator>::Item>>,
    rng: XorShiftRng,
}

impl<I> Shuffle<I>
where
    I: Iterator,
{
    fn new(mut iter: I, buffer_size: usize, seed: u64) -> Shuffle<I> {
        // NOTE: cannot do vec! here because Option<<I as Iterator>::Item> does not implement Clone
        let mut buffer = Vec::with_capacity(buffer_size);

        let mut i = 0;
        while i < buffer_size {
            let val = iter.next();
            if val.is_none() {
                break;
            } else {
                buffer.push(val);
                i += 1;
            }
        }

        Shuffle {
            iter,
            buffer_size,
            buffer,
            rng: XorShiftRng::seed_from_u64(seed),
        }
    }
}

impl<I> fmt::Debug for Shuffle<I>
where
    I: Iterator,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Shuffle {{ buffer_size: {}, rng: {:?} }}",
            self.buffer_size, self.rng
        )
    }
}

impl<I> Iterator for Shuffle<I>
where
    I: Iterator,
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer_size == 0 {
            None
        } else {
            let index = self.rng.gen_range(0, self.buffer_size);
            let val = self.buffer[index].take();

            let replace_val = self.iter.next();
            if replace_val.is_some() {
                self.buffer[index] = replace_val;
            } else {
                self.buffer[index] = self.buffer[self.buffer_size - 1].take();
                self.buffer_size -= 1;
            }

            val
        }
    }
}

/// Batch is an iterator that returns the contents of its inner iterator in batches
///
/// ```
/// use datasets::Dataset;
///
/// let vals: Vec<Vec<usize>> = (0..8).batch(5, false).collect();
///
/// assert_eq!(vals.len(), 2);
/// assert_eq!(vals[0], vec![0, 1, 2, 3, 4]);
/// assert_eq!(vals[1], vec![5, 6, 7]);
/// ```
#[derive(Debug)]
pub struct Batch<I>
where
    I: Iterator,
{
    iter: I,
    batch_size: usize,
    drop_remainder: bool,
}

impl<I> Iterator for Batch<I>
where
    I: Iterator,
{
    type Item = Vec<<I as Iterator>::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut i = 1;
        let val = self.iter.next();
        if val.is_none() {
            None
        } else {
            let mut v = Vec::with_capacity(self.batch_size);
            v.push(val.unwrap());

            while i < self.batch_size {
                match self.iter.next() {
                    Some(x) => v.push(x),
                    None => break,
                }

                i += 1;
            }

            if v.len() < self.batch_size && self.drop_remainder {
                None
            } else {
                Some(v)
            }
        }
    }
}
