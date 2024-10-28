use crate::{Matrix, Scalar};
use std::ops::Mul;


impl<T: Clone> Matrix<T> {
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

    pub fn row(&self, n: usize) -> Vec<T> {
        // Vec::new()
       return self.0[n].clone();
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        self.0.iter().map(|row| row[n].clone()).collect()
        // Vec::new()

    }
}

impl<T> Mul for Matrix<T> 
where
    T: Clone + std::ops::Add<Output = T> + std::ops::Mul<Output = T> + Default,
{
    type Output = Option<Matrix<T>>;

    fn mul(self, other: Matrix<T>) -> Self::Output {
        if self.number_of_cols() != other.number_of_rows() {
            return None; // Matrix multiplication is not possible
        }

        let mut result = vec![vec![T::default(); other.number_of_cols()]; self.number_of_rows()];

        for i in 0..self.number_of_rows() {
            for j in 0..other.number_of_cols() {
                let mut sum = T::default();
                for k in 0..self.number_of_cols() {
                    sum = sum + self.0[i][k].clone() * other.0[k][j].clone();
                }
                result[i][j] = sum;
            }
        }

        Some(Matrix(result))
    }
}
