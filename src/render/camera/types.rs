use std::ops::Sub;

use vello::kurbo::{Point, Vec2};

#[derive(Default, Clone, Copy)]
pub struct ScreenPoint(pub(super) Point);

impl ScreenPoint {
    #[must_use]
    pub fn new(point:impl Into<Point>) -> Self {
        Self(point.into())
    }

    #[must_use]
    pub fn to_vec2(self) -> ScreenVec2 {
        ScreenVec2(self.0.to_vec2())
    }
}

#[derive(Default, Clone, Copy)]
pub struct ScreenVec2(pub(super) Vec2);

impl ScreenVec2 {
    #[must_use]
    pub fn new(vec2:impl Into<Vec2>) -> Self {
        Self(vec2.into())
    }

    #[must_use]
    #[expect(unused)]
    pub fn to_point(self) -> ScreenPoint {
        ScreenPoint(self.0.to_point())
    }
}

impl Sub for ScreenVec2 {
    type Output = ScreenVec2;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}


