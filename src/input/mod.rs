use std::{collections::HashSet, ops::{BitOr, BitOrAssign}};

use educe::Educe;
use vello::kurbo::Vec2;
use winit::{event::{ElementState, KeyEvent, MouseButton}, keyboard::{KeyCode, PhysicalKey}};

use crate::game::GameState;
use crate::render::camera::{Camera, ScreenPoint};
use crate::state::State;

mod state;

pub mod button;

pub use state::InputState;

#[derive(Educe)]
#[educe(Default)]
pub struct UserState {
    pub mouse_position: ScreenPoint,
    pub camera_drag: bool,
    pub held_keys: HashSet<KeyCode>,
}

pub fn mouse_moved(to:ScreenPoint, state:&mut State) {
    let distance = to.to_vec2() - state.input_state.user_state.mouse_position.to_vec2();
    let user_state = &mut state.input_state.user_state;
    user_state.mouse_position = to;
    if user_state.camera_drag { state.render_state.camera.drag_input(distance); }
}

pub fn mouse_input(button_state:ElementState, mouse_button:MouseButton, state:&mut State) {
    let response = state.input_state.mouse_input(button_state, mouse_button, &state.render_state.camera, &mut state.game_state);

    let user_state = &mut state.input_state.user_state;
    user_state.camera_drag = match (button_state, mouse_button, response.hovered) {
        (ElementState::Pressed, MouseButton::Right, false) => true,
        (ElementState::Pressed, MouseButton::Middle, _) => true,
        (ElementState::Released, MouseButton::Right | MouseButton::Middle, _) => false,
        _ => user_state.camera_drag,
    }
}

pub fn mouse_scroll(amount:Vec2, state:&mut State) {
    state.render_state.camera.scroll_input(amount.y, state.input_state.user_state.mouse_position);
}

pub fn keyboard_input(event:KeyEvent, state:&mut State) {
    let KeyEvent { physical_key:PhysicalKey::Code(key), state:key_state, .. } = event else { return; };
    let held_keys = &mut state.input_state.user_state.held_keys;
    match key_state {
        ElementState::Pressed => {
            held_keys.insert(key);
            match key {
                KeyCode::KeyH => {
                    state.render_state.camera.home();
                },
                _ => {}
            }
        },
        ElementState::Released => {
            held_keys.remove(&key);
        }
    }

}

impl UserState {
    #[must_use]
    pub fn keyboard_input_direction(&self) -> Vec2 {
        let mut result = Vec2::ZERO;
        for (key, direction) in [
            (KeyCode::KeyW, Vec2::new(0., -1.)), (KeyCode::KeyA, Vec2::new(-1., 0.)),
            (KeyCode::KeyS, Vec2::new(0., 1.)), (KeyCode::KeyD, Vec2::new(1., 0.)),
        ] {
            result += self.held_keys.get(&key).and(Some(direction)).unwrap_or_default();
        }
        result
    }
}

#[derive(Default, Clone, Copy)]
pub struct MouseInputResponse {
    hovered: bool,
    consumed: bool,
}

impl BitOr for MouseInputResponse {
    type Output = MouseInputResponse;
    fn bitor(self, rhs: Self) -> Self::Output {
        MouseInputResponse {
            hovered: self.hovered | rhs.hovered,
            consumed: self.consumed | rhs.consumed,
        }
    }
}

impl BitOrAssign for MouseInputResponse {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs
    }
}

pub trait Inputable {
    type NewArgs;
    #[must_use]
    fn new(id:usize, args:Self::NewArgs) -> Self;

    fn update(&mut self, user_state:&UserState, game_state:&GameState, camera:&Camera) {
        let _ = user_state;
        let _ = game_state;
        let _ = camera;
    }

    #[must_use]
    fn mouse_input(
        &mut self, button_state:ElementState, mouse_button:MouseButton,
        user_state:&UserState, camera:&Camera, game_state:&mut GameState
    ) -> MouseInputResponse;
}

