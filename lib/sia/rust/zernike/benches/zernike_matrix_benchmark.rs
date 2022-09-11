use criterion::{criterion_group, criterion_main, Criterion};
use zernike::calc::get_zernike_matrix;

fn zernike_matrix_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("zernike matrix");

    group.bench_function("zernike matrix 21 0 64x64", |b| {
        b.iter(|| get_zernike_matrix(21, 0, 64))
    });
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().significance_level(0.1).sample_size(100);
    targets = zernike_matrix_benchmark
}

criterion_main!(benches);
