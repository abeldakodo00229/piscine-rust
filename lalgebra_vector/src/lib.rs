use std::ops::{Add, Mul};
use std::clone::Clone;
use std::cmp::{PartialEq, Eq};
pub trait Scalar: Add<Output = Self> + Mul<Output = Self> + Clone + PartialEq + std::fmt::Debug {}
impl<T: Add<Output = T> + Mul<Output = T> + Clone + PartialEq + std::fmt::Debug> Scalar for T {}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);
impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Self>;
    fn add(self, other: Self) -> Option<Self> {
        if self.0.len() != other.0.len() {
            return None;
        }
        let mut result = Vec::with_capacity(self.0.len());
        for (a, b) in self.0.into_iter().zip(other.0.into_iter()) {
            result.push(a + b);
        }
        Some(Vector(result))
    }
}
impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }
    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }
        let mut dot_product = self.0[0].clone() * other.0[0].clone();
        for i in 1..self.0.len() {
            dot_product = dot_product + self.0[i].clone() * other.0[i].clone();
        }
        Some(dot_product)
    }
}