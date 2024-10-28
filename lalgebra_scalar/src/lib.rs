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
