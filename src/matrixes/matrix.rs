use std::ops::{Add, Deref, DerefMut, Mul};

#[derive(Debug, Eq, PartialEq)]
pub struct Matrix<T, const M: usize, const N: usize> {
    matrix: [[T; N]; M],
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
    where
        T: Clone,
{
    pub fn new(initial_value: T) -> Self {
        Self {
            matrix: std::array::from_fn(|_| std::array::from_fn(|_| initial_value.clone())),
        }
    }

    pub fn transpose(&self) -> Matrix<T, N, M> {
        let mut matrix = Matrix::new(self[0][0].clone());

        for m in 0..M {
            for n in 0..N {
                matrix[n][m] = self[m][n].clone();
            }
        }

        matrix
    }
}

impl<T, O, const M: usize, const N: usize> Add for Matrix<T, M, N>
    where
        T: Add<Output=O> + Clone,
        O: Add + Mul + Clone,
{
    type Output = Matrix<O, M, N>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut matrix = Matrix::new(self[0][0].clone() + self[0][0].clone());

        for i in 0..M {
            for j in 0..N {
                matrix[i][j] = self[i][j].clone() + rhs[i][j].clone();
            }
        }

        matrix
    }
}

impl<T, O, const M: usize, const N: usize, const K: usize> Mul<Matrix<T, N, K>> for Matrix<T, M, N>
    where
        T: Mul<Output=O> + Clone,
        O: Default + Add<Output=O> + Clone,
{
    type Output = Matrix<O, M, K>;

    fn mul(self, rhs: Matrix<T, N, K>) -> Self::Output {
        let mut matrix = Matrix::default();

        for m in 0..M {
            for k in 0..K {
                let mut sum = O::default();
                for n in 0..N {
                    sum = sum + (self[m][n].clone() * rhs[n][k].clone());
                }
                matrix[m][k] = sum;
            }
        }

        matrix
    }
}

impl<T, const M: usize, const N: usize> From<[[T; N]; M]> for Matrix<T, M, N> {
    fn from(value: [[T; N]; M]) -> Self {
        Matrix { matrix: value }
    }
}

impl<T, const M: usize, const N: usize> Default for Matrix<T, M, N>
    where
        T: Default + Clone,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T, const M: usize, const N: usize> Clone for Matrix<T, M, N>
    where
        T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            matrix: self.matrix.clone(),
        }
    }
}

impl<T, const M: usize, const N: usize> Copy for Matrix<T, M, N> where T: Copy + Clone {}

impl<T, const M: usize, const N: usize> Deref for Matrix<T, M, N> {
    type Target = [[T; N]; M];

    fn deref(&self) -> &Self::Target {
        &self.matrix
    }
}

impl<T, const M: usize, const N: usize> DerefMut for Matrix<T, M, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.matrix
    }
}

#[cfg(test)]
mod test {
    use crate::matrixes::matrix::Matrix;

    #[test]
    fn zero_eq_def() {
        let m1: Matrix<usize, 2, 2> = Matrix::new(0);
        let m2: Matrix<usize, 2, 2> = Matrix::default();

        assert_eq!(m1, m2);
    }

    #[test]
    fn zero_plus_one() {
        let m0: Matrix<usize, 2, 2> = Matrix::default();
        let m1: Matrix<usize, 2, 2> = Matrix::new(1);

        assert_eq!(m0 + m1, m1);
    }

    #[test]
    fn zero_times_one_both() {
        let m0: Matrix<usize, 2, 2> = Matrix::default();
        let m1: Matrix<usize, 2, 2> = Matrix::new(1);

        assert_eq!(m0 * m1, m0);
    }

    #[test]
    fn zero_times_one() {
        let m0: Matrix<usize, 3, 2> = Matrix::default();
        let m1: Matrix<usize, 2, 1> = Matrix::new(1);

        assert_eq!(m0 * m1, Matrix::default());
    }

    #[test]
    fn one_minus_one() {
        let m1: Matrix<isize, 2, 2> = Matrix::new(1);
        let mm1: Matrix<isize, 2, 2> = Matrix::new(-1);

        assert_eq!(m1 + mm1, Matrix::default());
    }

    #[test]
    fn one_times_one() {
        let m1: Matrix<usize, 2, 2> = Matrix::new(1);

        assert_eq!(m1 * m1, Matrix::new(2));
    }
}
