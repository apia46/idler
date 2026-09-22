pub const BASE:f64 = f64::from_bits(0x5FF0000000000000); // 2^512
pub const R_BASE:f64 = 1./BASE;

// represents a number mantissa*BASE^exponent
// 0 is stored as (0, 0)
#[derive(Clone, Copy, Debug)]
pub struct Number {
    mantissa: f64, // (-2^512, -1] U [1, 2^512)
    exponent: f64, // integer
}

pub const NAN:Number = Number {mantissa: f64::NAN, exponent: f64::NAN};

impl Number {
    fn new(value: f64) -> Self {
        const MIN_POS:f64 = f64::from_bits(1);

        let sign:f64 = value.signum();
        let value = value.abs();
        match value {
            0.0 => Number { mantissa: 0.*sign, exponent: 0. },
            MIN_POS..R_BASE => Number { mantissa: value*BASE*BASE*sign, exponent: -2. },
            R_BASE..1.0 => Number { mantissa: value*BASE*sign, exponent: -1. },
            1.0..BASE => Number { mantissa: value*sign, exponent: 0. },
            BASE..f64::INFINITY => Number { mantissa: value/BASE*sign, exponent: 1. },
            f64::INFINITY => Number { mantissa: f64::INFINITY*sign, exponent: f64::INFINITY },
            _ => NAN,
        }
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

