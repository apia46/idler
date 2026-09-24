use std::ops::{Add, AddAssign, BitXor, BitXorAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Number(f64);

impl Number {
    pub fn new(value:f64) -> Self {
        Number(value.ln())
    }

    fn add_lesser(self, Number(value):Self) -> Self {
        Number(self.0 + f64::ln(1.+f64::exp(value-self.0)))
    }

    fn sub_lesser(self, Number(value):Self) -> Self {
        Number(self.0 + f64::ln(1.-f64::exp(value-self.0)))
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

impl Mul for Number {
    type Output = Number;
    fn mul(self, rhs: Self) -> Self::Output {
        Number(self.0+rhs.0)
    }
}

impl MulAssign for Number {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl Div for Number {
    type Output = Number;
    fn div(self, rhs: Self) -> Self::Output {
        Number(self.0-rhs.0)
    }
}

impl DivAssign for Number {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Add for Number {
    type Output = Number;
    fn add(self, rhs: Self) -> Self::Output {
        if rhs > self { rhs.add_lesser(self) }
        else { self.add_lesser(rhs) }
    }
}

impl AddAssign for Number {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}


impl Sub for Number {
    type Output = Number;
    fn sub(self, rhs: Self) -> Self::Output {
        if rhs > self { rhs.sub_lesser(self) }
        else { self.sub_lesser(rhs) }
    }
}

impl SubAssign for Number {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}


impl BitXor for Number {
    type Output = Number;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Number(self.0 * f64::exp(rhs.0))
    }
}

impl BitXorAssign for Number {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs
    }
}

impl ToString for Number {
    fn to_string(&self) -> String {
        self.0.exp().to_string()
    }
}
