use std::ops::{Add, Sub, Mul};

#[allow(unused)]
pub trait Lerp<T> {
    fn lerp(self, from: T, to: T) -> T;
}

macro_rules! lerp_impl {
    ($one:literal, $typ:ty) => {
        impl<T> Lerp<T> for $typ
        where T: Mul<$typ, Output=T> + Add<Output=T> + Sub<Output=T> {
            fn lerp(self, from: T, to: T) -> T {
                from * ($one - self) + to * self
            }
        }
    };
}

lerp_impl!(1., f32);
lerp_impl!(1., f64);

