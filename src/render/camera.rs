use std::time::Duration;

use educe::Educe;
use vello::kurbo::{Affine, Point, Vec2};

use crate::{input::UserState, util::Lerp};

const CAMERA_SPEED:f64 = 30000.;
const VELOCITY_DECAY:f64 = 1.5e-6;

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
    state: CameraState,
    #[educe(Default=1)]
    scale: f64,
}

#[derive(Educe)]
#[educe(Default)]
enum CameraState {
    #[educe(Default)]
    Normal(NormalCameraState),
    Homing(HomingCameraState),
}

#[derive(Educe)]
#[educe(Default)]
struct NormalCameraState {
    velocity: Vec2,
    zoom_point: ScreenPoint,
    #[educe(Default=1)]
    target_scale: f64,
}

#[derive(Default)]
struct HomingCameraState {
    target_pos: Point,
    target_scale: f64,
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

    pub fn process(&mut self, user_state:&UserState, dt:Duration) {
        let dt = dt.as_secs_f64();
        let adjusted_decay = VELOCITY_DECAY.powf(dt);

        if user_state.keyboard_input_direction() != Vec2::ZERO { self.cancel_homing(); };

        match &mut self.state {
            CameraState::Normal(state) => {
                state.velocity += user_state.keyboard_input_direction() * CAMERA_SPEED * self.scale * dt;
                let new_scale = (1.-adjusted_decay).lerp(self.scale, state.target_scale);
                // zoom correction: the mouse should be at the same world position before and after the zoom
                // mouse * scale_i + pos_i = mouse * scale_f + pos_f
                // mouse * (scale_i-scale_f) + pos_i = pos_f
                // zoom_correction = mouse * (scale_i-scale_f)
                let zoom_correction = state.zoom_point.to_vec2() * (self.scale - new_scale);
                state.velocity *= adjusted_decay;
                self.pos += state.velocity * dt + zoom_correction;
                self.scale = new_scale;
            },
            CameraState::Homing(state) => {
                self.pos = (1.-adjusted_decay).lerp(self.pos.to_vec2(), state.target_pos.to_vec2()).to_point();
                self.scale = (1.-adjusted_decay).lerp(self.scale, state.target_scale);
                if self.pos.distance_squared(state.target_pos) < 1.
                    && (self.scale - state.target_scale).abs() < 0.01 {
                    self.pos = state.target_pos;
                    self.state = CameraState::Normal(NormalCameraState { target_scale: state.target_scale, ..Default::default() });
                }
            },
        }
    }

    fn cancel_homing(&mut self) -> Option<&mut NormalCameraState> {
        match &mut self.state {
            CameraState::Normal(_) => {},
            CameraState::Homing(_) => { self.state = CameraState::Normal(NormalCameraState { target_scale: self.scale, ..Default::default() }); },
        }
        if let CameraState::Normal(state) = &mut self.state { Some(state) } else { None }
    }

    pub fn scroll_input(&mut self, amount:f64, mouse_position:ScreenPoint) {
        let Some(state) = self.cancel_homing() else { return; };
        state.zoom_point = mouse_position;
        state.target_scale *= 2_f64.powf(amount*-0.02);
        state.target_scale = state.target_scale.clamp(0.25, 16.);
    }

    pub fn drag_input(&mut self, distance:Vec2) {
        if let Some(_) = self.cancel_homing() {
            self.pos += -distance * self.scale;
        }
    }

    pub fn home(&mut self) {
        self.state = CameraState::Homing(HomingCameraState { target_pos: Point::new(0., 0.), target_scale: 1. });
    }
}

