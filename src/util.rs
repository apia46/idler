use std::ops::{Add, Div, Mul, Sub};

use vello::{kurbo::{Point, Vec2}, peniko::Color};

pub trait Lerp<T> {
    #[must_use]
    fn lerp(self, from:T, to:T) -> T;
}

pub trait Lerpable: Sized + Mul<f64, Output=Self> + Add<Output=Self> + Sub<Output=Self> {}

impl<T> Lerp<T> for f64 where T: Lerpable {
    fn lerp(mut self, from:T, to:T) -> T {
        self = self.clamp(0., 1.);
        from * (1. - self) + to * self
    }
}

impl Lerpable for f64 {}
impl Lerpable for Vec2 {}

impl Lerp<Point> for f64 {
    fn lerp(self, from:Point, to:Point) -> Point {
        self.lerp(from.to_vec2(), to.to_vec2()).to_point()
    }
}
impl Lerp<Color> for f64 {
    fn lerp(self, from:Color, to:Color) -> Color {
        let value = self.clamp(0., 1.) as f32;
        from * (1. - value) + to * value
    }
}

pub trait Remap: Sized {
    #[must_use]
    fn remap(self, from: (Self, Self), to: (Self, Self)) -> Self;
}

pub trait Remapable: Lerp<Self> + Sub<Output=Self> + Copy + Div<Output=Self> + num_traits::float::Float {}

impl<T> Remap for T where T: Remapable {
    fn remap(mut self, (from_low, from_high): (Self, Self), (to_low, to_high): (Self, Self)) -> Self {
        self = self.clamp(from_low, from_high);
        ((self - from_low) / (from_high - from_low)).lerp(to_low, to_high)
    }
}

impl Remapable for f64 {}

pub trait ComponentWise {
    #[expect(unused)]
    #[must_use]
    fn mul(self, value:Self) -> Self;
    #[must_use]
    fn div(self, value:Self) -> Self;
}

impl ComponentWise for Vec2 {
    fn mul(self, value:Self) -> Self {
        Vec2::new(self.x * value.x, self.y * value.y)
    }

    fn div(self, value:Self) -> Self {
        Vec2::new(self.x / value.x, self.y / value.y)
    }
}

/// multiplicative lerp
pub trait Merp<T> {
    #[must_use]
    fn merp(self, from:T, to:T) -> T;
}

