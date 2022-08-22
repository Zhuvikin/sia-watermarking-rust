use criterion::{criterion_group, criterion_main, Benchmark, Criterion};
use feature::moments::zernike::{calculate_zernike_image_moment, get_zernike_matrix};
use image::imageops::grayscale;
use image::{GenericImageView, GrayImage, ImageBuffer, Luma};
use ndarray::{Array2, ArrayBase, Ix2, OwnedRepr};
use std::time::Duration;

fn zernike_matrix_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("zernike matrix");

    group.bench_function("zernike matrix 21 0 64x64", |b| {
        b.iter(|| get_zernike_matrix(21, 0, 64))
    });
    group.bench_function("zernike matrix 21 1 64x64", |b| {
        b.iter(|| get_zernike_matrix(21, 1, 64))
    });
    group.bench_function("zernike matrix 21 2 64x64", |b| {
        b.iter(|| get_zernike_matrix(21, 2, 64))
    });
    group.bench_function("zernike matrix 21 3 64x64", |b| {
        b.iter(|| get_zernike_matrix(21, 3, 64))
    });
    group.bench_function("zernike matrix 21 4 64x64", |b| {
        b.iter(|| get_zernike_matrix(21, 4, 64))
    });

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().significance_level(0.1).sample_size(100);
    targets = zernike_matrix_benchmark
}

criterion_main!(benches);
