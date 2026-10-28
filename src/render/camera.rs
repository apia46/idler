use std::time::Duration;

use educe::Educe;
use vello::kurbo::{Affine, Point, Vec2};
use winit::keyboard::KeyCode;

use crate::{input::UserState, util::Lerp};

const CAMERA_SPEED:f64 = 30000.;
const LERP_AMOUNT:f64 = 0.2;

#[derive(Default, Clone, Copy)]
pub struct ScreenPoint(Point);

impl ScreenPoint {
    pub fn new(point:Point) -> Self {
        Self(point)
    }

    pub fn to_vec2(self) -> Vec2 {
        self.0.to_vec2()
    }
}

#[derive(Educe)]
#[educe(Default)]
pub struct Camera {
    pos: Point,
    velocity: Vec2,
    zoom_point: ScreenPoint,
    #[educe(Default=1)]
    target_scale: f64,
    #[educe(Default=1)]
    scale: f64,
}

impl Camera {
    pub fn to_world(&self, value:ScreenPoint) -> Point {
        self.transform() * value.0
    }

    pub fn transform(&self) -> Affine {
        Affine::scale(self.scale).then_translate(self.pos.to_vec2())
    }

    pub fn transform_inv(&self) -> Affine {
        self.transform().inverse()
    }

    pub fn process(&mut self, state:&UserState, dt:Duration) {
        for (key, direction) in [
            (KeyCode::KeyW, Vec2::new(0., -1.)), (KeyCode::KeyA, Vec2::new(-1., 0.)),
            (KeyCode::KeyS, Vec2::new(0., 1.)), (KeyCode::KeyD, Vec2::new(1., 0.)),
        ] {
            self.velocity += state.held_keys.get(&key).and(Some(direction)).unwrap_or_default() * CAMERA_SPEED * self.scale * dt.as_secs_f64();
        }
        let new_scale = LERP_AMOUNT.lerp(self.scale, self.target_scale);
        // zoom correction: the mouse should be at the same world position before and after the zoom
        // mouse * scale_i + pos_i = mouse * scale_f + pos_f
        // mouse * (scale_i-scale_f) + pos_i = pos_f
        // zoom_correction = mouse * (scale_i-scale_f)
        let zoom_correction = self.zoom_point.to_vec2() * (self.scale - new_scale);
        self.velocity *= 1.-LERP_AMOUNT;
        self.pos += self.velocity * dt.as_secs_f64() + zoom_correction;
        self.scale = new_scale;
    }

    pub fn scroll_input(&mut self, amount:f64, mouse_position:ScreenPoint) {
        self.zoom_point = mouse_position;
        self.target_scale *= 2_f64.powf(amount*-0.02);
        self.target_scale = self.target_scale.clamp(0.25, 16.);
    }

    pub fn drag_input(&mut self, distance:Vec2) {
        self.pos += -distance * self.scale;
    }
}

