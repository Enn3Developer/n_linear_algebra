use n_linear_algebra::matrix::matrix::Matrix;
use n_linear_algebra::matrix::square_matrix::SquareMatrix;
use rug::Integer;
use std::time::Instant;

fn main() {
    let m = Matrix::from([
        [Integer::from(1), Integer::from(1)],
        [Integer::from(1), Integer::from(0)],
    ]);
    let square_matrix = SquareMatrix::from(m);

    let fib = 1_000_000;
    let start = Instant::now();
    let _ = square_matrix.fast_pow(fib)[0][1].clone();
    let end = Instant::now();

    let time = end - start;

    println!("Fibonacci of {fib}, took {}s", time.as_secs_f64());
}
