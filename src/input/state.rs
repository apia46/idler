use winit::dpi::PhysicalPosition;

use crate::{game::GameState, input::button::Button};

#[derive(Default)]
pub struct UserState {
    pub mouse_position: PhysicalPosition<f64>,
}

#[derive(Default)]
pub struct InputState {
    pub user_state: UserState,
    pub buttons: Vec<Button>,
}

impl InputState {
    pub fn update(&mut self, state:&GameState) {
        for button in &mut self.buttons {
            button.update(&self.user_state, state);
        }
    }
}

