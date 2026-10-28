use std::{ops::{Add, AddAssign}, fmt::Display};

use crate::num::Number;

macro_rules! num_type {
    ($ty:ident) => {
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        pub struct $ty(Number);

        impl Display for $ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl $ty {
            pub const ONE:$ty = $ty(Number::ONE);

            pub fn to_number(self) -> Number {
                self.0
            }
        }
    };
}

macro_rules! boost_type {
    ($ty:ident) => {
        num_type!($ty);
        impl Add<$ty> for Quantity {
            type Output = Quantity;
            fn add(self, rhs:$ty) -> Self::Output {
                Quantity(self.0 + rhs.0)
            }
        }

        impl AddAssign<$ty> for Quantity {
            fn add_assign(&mut self, rhs:$ty) {
                *self = *self + rhs;
            }
        }

    };
}

num_type!(Quantity);
boost_type!(AddBoost);
boost_type!(MulBoost);
boost_type!(PowBoost);

