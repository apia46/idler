use winit::{event::{ElementState, MouseButton}, keyboard::KeyCode};

pub use state::InputState;
use crate::game::GameState;

mod state;
pub mod button;

pub fn mouse_input(button_state:ElementState, mouse_button:MouseButton, input_state:&mut InputState, game_state:&mut GameState) {
    for button in &mut input_state.buttons {
        button.mouse_input(button_state, mouse_button, game_state);
    }
}

pub fn keyboard_input(key:KeyCode, _state:&mut GameState) {
    match key {
        _ => {}
    }
}

