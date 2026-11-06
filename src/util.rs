use std::ops::{Add, Sub, Mul};

use vello::kurbo::{Point, Vec2};

pub trait Lerp<T> {
    fn lerp(self, from: T, to: T) -> T;
}

pub trait Lerpable: Sized + Mul<f64, Output=Self> + Add<Output=Self> + Sub<Output=Self> {}

impl<T> Lerp<T> for f64 where T: Lerpable {
    fn lerp(self, from: T, to: T) -> T {
        from * (1. - self) + to * self
    }
}

impl Lerpable for f64 {}
impl Lerpable for Vec2 {}

impl Lerp<Point> for f64 {
    fn lerp(self, from: Point, to: Point) -> Point {
        self.lerp(from.to_vec2(), to.to_vec2()).to_point()
    }
}

