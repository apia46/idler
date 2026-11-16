use std::{fmt::Display, ops::{Add, AddAssign, Mul, Sub, SubAssign, BitXor}, cmp::Ordering};

use crate::num::Number;

macro_rules! operation {
    ($op_trait:ident : $lhs:ty [$op_fn:ident] $rhs:ty => $out:ident with |$a:ident, $b:ident| $expr:expr) => {
        impl $op_trait<$rhs> for $lhs {
            type Output = $out;
            fn $op_fn(self, $b:$rhs) -> Self::Output {
                let $a = self;
                $expr
            }
        }
    };
    ($op_trait:ident : Number [$op_fn:ident] $rhs:ty => $out:ident) => {
        operation!($op_trait : Number [$op_fn] $rhs => $out with |a, b| $out(a.$op_fn(b.0)));
    };
    ($op_trait:ident : $lhs:ty [$op_fn:ident] Number => $out:ident) => {
        operation!($op_trait : $lhs [$op_fn] Number => $out with |a, b| $out(a.0.$op_fn(b)));
    };
    ($op_trait:ident : $lhs:ty [$op_fn:ident] $rhs:ty => $out:ident) => {
        operation!($op_trait : $lhs [$op_fn] $rhs => $out with |a, b| $out(a.0.$op_fn(b.0)));
    }
}

macro_rules! comparison {
    (singular, $lhs:ty, $rhs:ty, |$a:ident| $a_expr:expr, |$b:ident| $b_expr:expr) => {
        impl PartialEq<$rhs> for $lhs {
            fn eq(&self, $b:&$rhs) -> bool {
                let $a = self;
                $a_expr.eq($b_expr)
            }
        }

        impl PartialOrd<$rhs> for $lhs {
            fn partial_cmp(&self, $b:&$rhs) -> Option<Ordering> {
                let $a = self;
                $a_expr.partial_cmp($b_expr)
            }
        }
    };
    ($lhs:ty, $rhs:ty, |$a:ident| $a_expr:expr, |$b:ident| $b_expr:expr) => {
        comparison!(singular, $lhs, $rhs, |$a| $a_expr, |$b| $b_expr);

    };
    (Number, $rhs:ty) => {
        comparison!(Number, $rhs, |a| a, |b| &b.0);
    };
    ($lhs:ty, Number) => {
        comparison!($lhs, Number, |a| &a.0, |b| b);
    };
    ($lhs:ty, $rhs:ty) => {
        comparison!($lhs, $rhs, |a| a.0, |b| &b.0);
    }
}

comparison!(Number, Number, |a| a.0, |b| &b.0);
comparison!(Number, f64, |a| a, |b| &Number::from(*b));
comparison!(Number, i32, |a| a, |b| &Number::from(*b));

macro_rules! symm_comparison {
    (Number, $rhs:ty) => {
        comparison!(Number, $rhs);
        comparison!($rhs, Number);
    };
    ($lhs:ty, Number) => {
        comparison!($lhs, Number);
        comparison!(Number, $lhs);
    };
    ($lhs:ty, $rhs:ty) => {
        comparison!($lhs, $rhs);
        comparison!($rhs, $lhs);
    }
}

macro_rules! default_zero {
    ($ty:ident) => {
        impl $ty {
            #[allow(unused)]
            pub const ZERO:$ty = $ty(Number::ZERO);
        }

        impl Default for $ty { fn default() -> Self { $ty::ZERO } }
    };
}

macro_rules! num_type {
    ($ty:ident) => {
        #[derive(Clone, Copy, Debug)]
        pub struct $ty(Number);

        symm_comparison!($ty, Number);
        comparison!($ty, f64, |a| a, |b| &Number::from(*b));
        comparison!($ty, i32, |a| a, |b| &Number::from(f64::from(*b)));

        impl Display for $ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl From<Number> for $ty {
            fn from(value:Number) -> Self {
                Self(value)
            }
        }

        impl From<f64> for $ty {
            fn from(value:f64) -> Self {
                Self(Number::new(value))
            }
        }

        impl From<i32> for $ty {
            fn from(value:i32) -> Self {
                Self(Number::new(value.into()))
            }
        }

        impl $ty {
            #[allow(unused)]
            pub const ONE:$ty = $ty(Number::ONE);

            #[must_use]
            #[allow(unused)]
            pub fn new(value:f64) -> Self { Self(Number::new(value)) }

            #[must_use]
            #[allow(unused)]
            pub fn to_number(self) -> Number {
                self.0
            }

            #[must_use]
            #[allow(unused)]
            pub fn as_number(&mut self) -> &mut Number {
                &mut self.0
            }
        }

        impl AddAssign<$ty> for Number {
            fn add_assign(&mut self, rhs:$ty) {
                *self = *self + rhs.0;
            }
        }

        impl SubAssign<$ty> for Number {
            fn sub_assign(&mut self, rhs:$ty) {
                *self = *self - rhs.0;
            }
        }

        impl AddAssign<Number> for $ty {
            fn add_assign(&mut self, rhs:Number) {
                self.0 = self.0 + rhs;
            }
        }

        impl SubAssign<Number> for $ty {
            fn sub_assign(&mut self, rhs:Number) {
                self.0 = self.0 - rhs;
            }
        }
    };
    ($ty:ident, ZERO) => {
        num_type!($ty);
        default_zero!($ty);
    };
    ($ty:ident, ONE) => {
        num_type!($ty);
        default_one!($ty);
    };
}

macro_rules! num_operation {
    ($op_trait:ident : $lhs:ty [$op_fn:ident] $rhs:ty => $out:ident) => {
        operation!($op_trait : $lhs [$op_fn] $rhs => $out with |a, b| $out(a.0.$op_fn(b.0)));
    }
}

macro_rules! final_type {
    ($ty:ident) => {
        num_operation!(Add: Quantity [add] $ty => Quantity);
        num_operation!(Sub: Quantity [sub] $ty => Quantity);

        impl AddAssign<$ty> for Quantity {
            fn add_assign(&mut self, rhs:$ty) {
                *self = *self + rhs;
            }
        }

        impl SubAssign<$ty> for Quantity {
            fn sub_assign(&mut self, rhs:$ty) {
                *self = *self - rhs;
            }
        }
    };
}

macro_rules! boost {
    ($fn:ident, $op:ident) => {
        #[allow(unused)]
        pub fn $fn(self, active:impl Into<bool>, value:impl Into<Number>) -> Self {
            if active.into() { self.$op(value.into()) } else { self }
        }
    }
}

macro_rules! boost_impl {
    ($fn:ident) => {
        #[allow(unused)]
        pub fn $fn(self, active:impl Into<bool>, value:impl Into<Number>) -> Self {
            Self(self.0.$fn(active, value.into()))
        }
    }
}


impl Number {
    boost!(add_boost, add);
    boost!(mul_boost, mul);
    boost!(pow_boost, bitxor);
}

macro_rules! boost_type {
    ($ty:ident) => {
        impl $ty {
            boost_impl!(add_boost);
            boost_impl!(mul_boost);
            boost_impl!(pow_boost);
        }
    }
}

num_type!(Quantity);

num_type!(RateOfChange, ZERO);
num_type!(Cost, ZERO);
num_type!(Gain, ZERO);

boost_type!(RateOfChange);
boost_type!(Cost);
boost_type!(Gain);

final_type!(Cost);
final_type!(Gain);

symm_comparison!(Quantity, Cost);

impl Quantity { #[allow(unused)] pub const ZERO:Quantity = Quantity(Number::ZERO); }

operation!(Mul: RateOfChange [mul] f64 => Gain with |a,b| Gain(a.0 * Number::from(b)) );

