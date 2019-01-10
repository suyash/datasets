//! mnist

use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use crate::utils::download;
use crate::Dataset;

/// downloads the mnist dataset to the given `download_dir` and loads from it.
/// This function returns a tuple with the train and test datasets respectively.
///
/// For the images, this returns a single vector of size 28 * 28 instead of a Vec<Vec<u8>>.
/// The 2D version adds overhead for tensor containers that are *almost* always 1D.
///
/// To get to 2D from 1D
///
/// let mut iter = img1D.iter();
/// let img2D = (0..28).map(|_| iter.take(28).collect()).collect();
pub fn load(
    download_dir: &Path,
) -> Result<
    (
        impl Dataset<Item = (Vec<u8>, u8)>,
        impl Dataset<Item = (Vec<u8>, u8)>,
    ),
    Box<dyn Error>,
> {
    download(
        "http://yann.lecun.com/exdb/mnist/train-images-idx3-ubyte.gz",
        download_dir,
        true,
    )?;
    download(
        "http://yann.lecun.com/exdb/mnist/train-labels-idx1-ubyte.gz",
        download_dir,
        true,
    )?;
    download(
        "http://yann.lecun.com/exdb/mnist/t10k-images-idx3-ubyte.gz",
        download_dir,
        true,
    )?;
    download(
        "http://yann.lecun.com/exdb/mnist/t10k-labels-idx1-ubyte.gz",
        download_dir,
        true,
    )?;

    Ok((
        extract_dataset(
            &download_dir.join("train-images.idx3-ubyte"),
            &download_dir.join("train-labels.idx1-ubyte"),
            60000,
        )?,
        extract_dataset(
            &download_dir.join("t10k-images.idx3-ubyte"),
            &download_dir.join("t10k-labels.idx1-ubyte"),
            60000,
        )?,
    ))
}

fn extract_dataset(
    features_path: &Path,
    labels_path: &Path,
    size: usize,
) -> Result<impl Dataset<Item = (Vec<u8>, u8)>, Box<dyn Error>> {
    let (mut features, mut labels) = (File::open(features_path)?, File::open(labels_path)?);

    features.seek(SeekFrom::Start(16))?;
    labels.seek(SeekFrom::Start(8))?;

    Ok(MNISTDataset::new(features, labels, size))
}

#[derive(Debug)]
struct MNISTDataset {
    features: File,
    labels: File,
    size: usize,
    current: usize,
    image_buffer: Vec<u8>,
    label_buffer: Vec<u8>,
}

impl MNISTDataset {
    fn new(features: File, labels: File, size: usize) -> MNISTDataset {
        let (image_buffer, label_buffer) = (vec![0; 28 * 28], vec![0; 1]);

        MNISTDataset {
            features,
            labels,
            size,
            current: 0,
            image_buffer,
            label_buffer,
        }
    }
}

impl Iterator for MNISTDataset {
    type Item = (Vec<u8>, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.size {
            None
        } else {
            let (r1, r2) = (
                self.features.read_exact(&mut self.image_buffer),
                self.labels.read_exact(&mut self.label_buffer),
            );

            if r1.is_err() || r2.is_err() {
                return None;
            }

            self.current += 1;

            Some((self.image_buffer.clone(), self.label_buffer[0]))
        }
    }
}
