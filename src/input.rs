use winit::{event::{ElementState, MouseButton}, keyboard::KeyCode};

use crate::state::State;

pub fn mouse_input(button_state:ElementState, button:MouseButton, state:&mut State) {
    if let ElementState::Released = button_state { return; };
    match button {
        MouseButton::Left => lmb(state),
        _ => {}
    }
}

pub fn keyboard_input(key:KeyCode, state:&mut State) {
    match key {
        KeyCode::Space => space(state),
        _ => {}
    }
}

pub fn lmb(state:&mut State) {
    state.things_per_second += state.things;
    state.things = 0.into();
    state.update::<true>(0.);
}

pub fn space(state:&mut State) {
    state.things_mult += state.things_per_second.safe_ln().into();
    state.things_per_second = 0.into();
    state.things = 1.into();
    state.update::<true>(0.);
}
