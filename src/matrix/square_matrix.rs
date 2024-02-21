use crate::matrix::matrix::Matrix;
use std::ops::{Add, Deref, DerefMut, Mul};

#[derive(Debug, Eq, PartialEq)]
pub struct SquareMatrix<T, const M: usize>(Matrix<T, M, M>);

impl<T, const M: usize> SquareMatrix<T, M>
    where
        T: Clone,
{
    pub fn fast_pow(self, exp: usize) -> Self
        where
            T: Mul<Output=T> + Add<Output=T> + Default,
    {
        if exp == 1 {
            self
        } else if exp % 2 == 0 {
            let m = self.fast_pow(exp / 2);
            m.clone() * m
        } else {
            let m = self.clone().fast_pow((exp - 1) / 2);
            self * (m.clone() * m)
        }
    }
}

impl<T, const M: usize, const N: usize> From<Matrix<T, M, N>> for SquareMatrix<T, M>
    where
        T: Default + Clone,
{
    fn from(value: Matrix<T, M, N>) -> Self {
        let mut matrix = Matrix::default();

        for m in 0..M {
            for n in 0..N {
                matrix[m][n] = value[m][n].clone();
            }
        }

        Self { 0: matrix }
    }
}

impl<T, const M: usize> Into<Matrix<T, M, M>> for SquareMatrix<T, M>
    where
        T: Default + Copy,
{
    fn into(self) -> Matrix<T, M, M> {
        let mut matrix = Matrix::default();

        for m in 0..M {
            for n in 0..M {
                matrix[m][n] = self[m][n];
            }
        }

        matrix
    }
}

impl<T, const M: usize> Mul for SquareMatrix<T, M>
    where
        T: Mul<Output=T> + Clone + Add<Output=T> + Default,
        SquareMatrix<T, M>: From<Matrix<T, M, M>>,
{
    type Output = SquareMatrix<T, M>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from(self.0 * rhs.0)
    }
}

impl<T, const M: usize> Clone for SquareMatrix<T, M>
    where
        T: Clone,
{
    fn clone(&self) -> Self {
        Self { 0: self.0.clone() }
    }
}

impl<T, const M: usize> Copy for SquareMatrix<T, M> where T: Copy + Clone {}

impl<T, const M: usize> Deref for SquareMatrix<T, M> {
    type Target = Matrix<T, M, M>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const M: usize> DerefMut for SquareMatrix<T, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
