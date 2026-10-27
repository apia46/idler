use winit::{event::{ElementState, MouseButton}, keyboard::KeyCode};

pub use state::InputState;
use crate::game::GameState;

mod state;
pub mod button;

pub fn mouse_input(button_state:ElementState, button:MouseButton, input_state:&mut InputState, logic_state:&mut GameState) {
    if let ElementState::Released = button_state { return; };
    match button {
        MouseButton::Left => lmb(input_state, logic_state),
        _ => {}
    }
}

pub fn keyboard_input(key:KeyCode, _state:&mut GameState) {
    match key {
        _ => {}
    }
}

pub fn lmb(input_state:&mut InputState, logic_state:&mut GameState) {
    for button in &input_state.buttons {
        if button.hovered(&input_state) {
            (button.pressed)(logic_state);
        }
    }
}

