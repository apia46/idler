use std::ops::{Add, AddAssign, Mul};

use vello::kurbo::{Affine, BezPath, PathEl, Point, Vec2};

const HALF_WIDTH:f64 = 50.;
const DIAG_WIDTH:f64 = 50.;
const HALF_HEIGHT:f64 = 70.;

const I_HAT:Vec2 = Vec2::new(2.*HALF_WIDTH+DIAG_WIDTH, HALF_HEIGHT);
const J_HAT:Vec2 = Vec2::new(0., 2.*HALF_HEIGHT);
const MATRIX:Affine = Affine::new([
    I_HAT.x, I_HAT.y, J_HAT.x, J_HAT.y, 0., 0.,
]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridPoint {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction { UP, RUP, ROWN, DOWN, LOWN, LUP }

impl GridPoint {
    pub fn new(x:i32, y:i32) -> GridPoint {
        GridPoint { x, y }
    }

    pub fn nearest(point: Point) -> GridPoint {
        let Point { x, y } = MATRIX.inverse() * point;
        let mut point = GridPoint { x: x.round() as i32, y: y.round() as i32 };
        let (rx, ry) = ((x+0.5).rem_euclid(1.), (y+0.5).rem_euclid(1.));
        // this point is the point in the square grid of the matrix thing
        // now we need to correct it to the nearest hexagon
        // see assets/explanation.png (its flipped vertically because of desmos)
        const SLOPE:f64 = I_HAT.x / I_HAT.y;
        if ry > rx {
            if ry < 0.5 - SLOPE*rx { point.x -= 1 }
            if 1.-rx < 0.5 - SLOPE*(1.-ry) { point.y += 1 }
        } else {
            if rx < 0.5 - SLOPE*ry { point.y -= 1 }
            if 1.-ry < 0.5 - SLOPE*(1.-rx) { point.x += 1 }
        }
        point
    }

    pub fn to_point(self) -> Point {
        (I_HAT * self.x as f64 + J_HAT * self.y as f64).to_point()
    }

    pub fn to_hexagon(self) -> BezPath {
        let center = self.to_point();
        BezPath::from_vec(vec![
            PathEl::MoveTo(center + Vec2::new(-HALF_WIDTH, -HALF_HEIGHT)),
            PathEl::LineTo(center + Vec2::new(HALF_WIDTH, -HALF_HEIGHT)),
            PathEl::LineTo(center + Vec2::new(HALF_WIDTH+DIAG_WIDTH, 0.)),
            PathEl::LineTo(center + Vec2::new(HALF_WIDTH, HALF_HEIGHT)),
            PathEl::LineTo(center + Vec2::new(-HALF_WIDTH, HALF_HEIGHT)),
            PathEl::LineTo(center + Vec2::new(-HALF_WIDTH-DIAG_WIDTH, 0.)),
            PathEl::LineTo(center + Vec2::new(-HALF_WIDTH, -HALF_HEIGHT)),
        ])
    }
}

impl From<GridPoint> for Point {
    fn from(value: GridPoint) -> Self {
        value.to_point()
    }
}

impl Add for GridPoint {
    type Output = GridPoint;
    fn add(self, rhs: Self) -> Self::Output {
        GridPoint::new(self.x+rhs.x, self.y+rhs.y)
    }
}

impl AddAssign for GridPoint {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Mul<u32> for Direction {
    type Output = GridPoint;
    fn mul(self, rhs: u32) -> Self::Output {
        let (x, y) = match self {
            Direction::UP => (0, -1),
            Direction::RUP => (1, -1),
            Direction::ROWN => (1, 0),
            Direction::DOWN => (0, 1),
            Direction::LOWN => (-1, 1),
            Direction::LUP => (-1, 0),
        };
        GridPoint::new(x*rhs as i32, y*rhs as i32)
    }
}

