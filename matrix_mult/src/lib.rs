pub mod mult;
pub use  mult::*;

pub trait Scalar{
    type Item;
    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}

macro_rules! impl_scalar {
    ($type:ty) => {
        impl Scalar for $type {
            type Item = Self;
            
            fn zero() -> Self::Item {
                0 as Self
            }
            
            fn one() -> Self::Item {
                1 as Self
            }
        }
    };
}

impl_scalar!(u32);
impl_scalar!(u64);
impl_scalar!(i32);
impl_scalar!(i64);
impl_scalar!(f32);
impl_scalar!(f64);

#[derive(Debug, PartialEq, Eq,Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);
impl<T: Scalar<Item = T> + Default> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::Item::default()]])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        let mut matrix = Vec::with_capacity(row);
        for _ in 0..row {
            let mut row_vec = Vec::with_capacity(col);
            for _ in 0..col {
                row_vec.push(T::Item::default());
            }
            matrix.push(row_vec);
        }
        Matrix(matrix)
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut matrix = Vec::with_capacity(n);
        for i in 0..n {
            let mut row_vec = Vec::with_capacity(n);
            for j in 0..n {
                if i == j {
                    row_vec.push(T::Item::one());
                } else {
                    row_vec.push(T::Item::default());
                }
            }
            matrix.push(row_vec);
        }
        Matrix(matrix)
    }
}

