use educe::Educe;
use vello::kurbo::{Affine, Point, Vec2};

use crate::{input::UserState, util::Lerp, window::Window};

const CAMERA_SPEED:f64 = 30000.;
const VELOCITY_DECAY:f64 = 1.5e-6;

mod types;

pub use types::{ScreenPoint, ScreenVec2};

#[derive(Educe)]
#[educe(Default)]
pub struct Camera {
    pub pos: Point,
    state: CameraState,
    #[educe(Default=1)]
    scale: f64,
    pub camera_moved: bool,
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
    #[must_use]
    pub fn point_to_world(&self, value:ScreenPoint) -> Point {
        self.transform() * value.0
    }
    #[must_use]
    pub fn vec2_to_world(&self, value:ScreenVec2) -> Vec2 {
        value.0 * self.scale
    }

    #[must_use]
    pub fn transform(&self) -> Affine {
        Affine::scale(self.scale).then_translate(self.pos.to_vec2())
    }

    #[must_use]
    pub fn transform_inv(&self) -> Affine {
        self.transform().inverse()
    }

    fn default_pos(&self, window:&Window) -> Point {
        // (at the default scale)
        (window.size.0 * -0.5).to_point()
    }

    pub fn go_to_default_pos(&mut self, window:&Window) {
        self.cancel_homing();
        self.pos = self.default_pos(window);
    }

    pub fn process(&mut self, user_state:&UserState, delta_time:f64) {
        let adjusted_decay = VELOCITY_DECAY.powf(delta_time);

        if user_state.keyboard_input_direction() != Vec2::ZERO { self.cancel_homing(); };

        let old_pos = self.pos;

        match &mut self.state {
            CameraState::Normal(state) => {
                state.velocity += user_state.keyboard_input_direction() * CAMERA_SPEED * self.scale * delta_time;
                let new_scale = (1.-adjusted_decay).lerp(self.scale, state.target_scale);
                // zoom correction: the mouse should be at the same world position before and after the zoom
                // mouse * scale_i + pos_i = mouse * scale_f + pos_f
                // mouse * (scale_i-scale_f) + pos_i = pos_f
                // zoom_correction = mouse * (scale_i-scale_f)
                let zoom_correction = state.zoom_point.to_vec2().0 * (self.scale - new_scale);
                state.velocity *= adjusted_decay;
                self.pos += state.velocity * delta_time + zoom_correction;
                self.scale = new_scale;
            },
            CameraState::Homing(state) => {
                self.pos = (1.-adjusted_decay).lerp(self.pos, state.target_pos);
                self.scale = (1.-adjusted_decay).lerp(self.scale, state.target_scale);
                if self.pos.distance_squared(state.target_pos) < 1.
                    && (self.scale - state.target_scale).abs() < 0.01 {
                    self.pos = state.target_pos;
                    self.state = CameraState::Normal(NormalCameraState { target_scale: state.target_scale, ..Default::default() });
                }
            },
        }

        self.camera_moved = old_pos != self.pos;
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

    pub fn drag_input(&mut self, distance:ScreenVec2) {
        if let Some(_) = self.cancel_homing() {
            self.pos += -distance.0 * self.scale;
        }
    }

    pub fn home(&mut self, window:&Window) {
        self.state = CameraState::Homing(HomingCameraState { target_pos: self.default_pos(window), target_scale: 1. });
    }
}

