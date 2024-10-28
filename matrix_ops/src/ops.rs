use crate::{Matrix, Scalar};
use std::ops::{Add, Sub};



impl<T> Add for Matrix<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Output = Option<Matrix<T>>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() || self.0[0].len() != rhs.0[0].len() {
            return None; 
        }

        let mut result = Vec::new();

        for i in 0..self.0.len() {
            let mut row = Vec::new();
            for j in 0..self.0[0].len() {
                row.push(self.0[i][j] + rhs.0[i][j]);
            }
            result.push(row);
        }

        Some(Matrix(result))
    }
}


impl<T> Sub for Matrix<T>
where
    T: std::ops::Sub<Output = T> + Copy,
{
    type Output = Option<Matrix<T>>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() || self.0[0].len() != rhs.0[0].len() {
            return None; 
        }

        let mut result = Vec::new();

        for i in 0..self.0.len() {
            let mut row = Vec::new();
            for j in 0..self.0[0].len() {
                row.push(self.0[i][j] - rhs.0[i][j]);
            }
            result.push(row);
        }

        Some(Matrix(result))
    }
}
