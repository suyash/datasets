#![feature(test)]

extern crate test;

use std::fs;
use std::path::Path;

use test::{black_box, Bencher};

use datasets::image::mnist;

#[bench]
fn mnist_load_from_scratch(b: &mut Bencher) {
    let path = Path::new("./tmp/mnist");
    match path.metadata() {
        Ok(d) => {
            if d.is_dir() {
                fs::remove_dir_all(path).unwrap();
            } else {
                fs::remove_file(path).unwrap();
            }

            fs::create_dir_all(path).unwrap();
        }
        Err(_) => fs::create_dir_all(path).unwrap(),
    }

    b.iter(|| {
        black_box(mnist::load(path).unwrap());
    });
}

#[bench]
fn mnist_load_from_cache(b: &mut Bencher) {
    let path = Path::new("./tmp/mnist");
    let _ = mnist::load(path).unwrap();

    b.iter(|| {
        black_box(mnist::load(path).unwrap());
    });
}

#[bench]
fn mnist_load_train_dataset_full(b: &mut Bencher) {
    let path = Path::new("./tmp/mnist");

    // make sure the dataset is downloaded
    let _ = mnist::load(path).unwrap();

    b.iter(|| {
        let (train_dataset, _) = mnist::load(path).unwrap();
        let v = train_dataset.collect::<Vec<(Vec<u8>, u8)>>();
        v.len()
    });
}

#[bench]
fn mnist_load_test_dataset_full(b: &mut Bencher) {
    let path = Path::new("./tmp/mnist");

    // make sure the dataset is downloaded
    let _ = mnist::load(path).unwrap();

    b.iter(|| {
        let (_, test_dataset) = mnist::load(path).unwrap();
        let v = test_dataset.collect::<Vec<(Vec<u8>, u8)>>();
        v.len()
    });
}
