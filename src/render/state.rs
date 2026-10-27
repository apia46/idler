use std::time::Duration;

use crate::{game::GameState, render::button::Button};

#[derive(Default)]
pub struct RenderState {
    pub previous_frame: Duration,
    pub delta_time: f64,
    pub buttons: Vec<Button>,
}

impl RenderState {
    pub fn update(&mut self, state: &GameState) {
        for button in &mut self.buttons {
            button.update(&state);
        }
    }
}

