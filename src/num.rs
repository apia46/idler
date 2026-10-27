use std::{f64::consts::LN_10, fmt::Display, ops::{Add, AddAssign, BitXor, BitXorAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

const LN_INF:f64 = 709.78271289338399684324569237317280570931213758490138021957632972;

const LN_EPSILON:f64 = -36.; // less than 10^-15

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Number(f64);

impl Number {
    pub const ZERO:Number = Number(f64::NEG_INFINITY);
    pub const ONE:Number = Number(0.);

    pub fn new(value:f64) -> Self {
        Number(value.ln())
    }

    fn add_lesser(self, Number(value):Self) -> Self {
        if value-self.0 < LN_EPSILON { self }
        else { Number(self.0 + f64::ln(1.+f64::exp(value-self.0))) }
    }

    fn sub_lesser(self, Number(value):Self) -> Self {
        if value-self.0 < LN_EPSILON { self }
        else { Number(self.0 + f64::ln(1.-f64::exp(value-self.0))) }
    }

    pub fn safe_ln(self) -> f64 {
        self.0.max(0.)
    }
}

impl From<f64> for Number {
    fn from(value:f64) -> Self {
        Self::new(value)
    }
}

impl From<i32> for Number {
    fn from(value:i32) -> Self {
        Self::new(value.into())
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

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&if self.0 < LN_10*9. {
            format!("{:.3}", self.0.exp())
        } else if self.0 < LN_INF {
            let exponent = self.0 / LN_10;
            let mantissa = 10_f64.powf(exponent.fract());
            format!("{mantissa:.3}e{}", exponent.floor())
        } else {
            format!("e{:.3}", self.0 / LN_10)
        })
    }
}

