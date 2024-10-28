use crate::{Matrix as OtherMatrix, Scalar};
use std::ops::Mul;


#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.0[0].len()
        }
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<&T> {
        if n < self.number_of_rows() {
            self.0[n].iter().collect()
        } else {
            Vec::new()
        }
    }

    pub fn col(&self, n: usize) -> Vec<&T> {
        if self.0.is_empty() {
            Vec::new()
        } else {
            self.0.iter().map(|row| &row[n]).collect()
        }
    }
}

impl<T: Copy + std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Default> Mul for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        if self.number_of_cols() != rhs.number_of_rows() {
            return None;
        }

        let mut result = vec![vec![T::default(); rhs.number_of_cols()]; self.number_of_rows()];

        for i in 0..self.number_of_rows() {
            for j in 0..rhs.number_of_cols() {
                for k in 0..self.number_of_cols() {
                    result[i][j] = result[i][j] + self.0[i][k] * rhs.0[k][j];
                }
            }
        }

        Some(Matrix(result))
    }
}
