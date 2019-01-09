#![feature(test)]

extern crate test;

use test::{black_box, Bencher};

use datasets::Dataset;

#[bench]
fn bench_shuffle(b: &mut Bencher) {
    b.iter(|| {
        black_box((0..2048).shuffle(256, 0).collect::<Vec<usize>>());
    });
}

#[bench]
fn bench_batch(b: &mut Bencher) {
    b.iter(|| {
        black_box((0..2048).batch(255).collect::<Vec<Vec<usize>>>());
    });
}
