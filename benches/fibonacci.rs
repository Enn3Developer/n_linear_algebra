use criterion::{black_box, criterion_group, criterion_main, Criterion};
use n_linear_algebra::matrixes::square_matrix::SquareMatrix;
use rug::Integer;

fn fibonacci(n: usize) -> Integer {
    let square_matrix = SquareMatrix::from([
        [Integer::from(1), Integer::from(1)],
        [Integer::from(1), Integer::from(0)],
    ]);

    square_matrix.fast_pow(n)[0][1].clone()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fibonacci", |b| b.iter(|| fibonacci(black_box(1000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
