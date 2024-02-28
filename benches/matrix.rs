use criterion::{black_box, criterion_group, criterion_main, Criterion};
use n_linear_algebra::matrixes::matrix::{Matrix, MulSimd};

fn matrix_multiplication(a: Matrix<usize, 8, 8>, b: Matrix<usize, 8, 8>) -> Matrix<usize, 8, 8> {
    a.mul_simd(b)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("matrixes multiplication", |b| {
        b.iter(|| matrix_multiplication(black_box(Matrix::new(3)), black_box(Matrix::new(5))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
