use winit::event::{ElementState, MouseButton};

use crate::game::GameState;
use crate::grid::GridPoint;
use crate::input::UserState;
use crate::render::camera::Camera;

#[derive(Clone, Copy)]
pub enum InteractState { Disabled, None, Hovered, Pressed }

pub struct Button {
    #[expect(unused)]
    id: usize,
    position: GridPoint,
    enabled: fn(&GameState) -> bool,
    pressed: fn(&mut GameState),
    pub state: InteractState,
}

pub struct NewButtonArgs {
    pub position: GridPoint,
    pub enabled: fn(&GameState) -> bool,
    pub pressed: fn(&mut GameState),
}

impl Button {
    #[must_use]
    pub fn new(id: usize, args: NewButtonArgs) -> Self {
        let NewButtonArgs { position, pressed, enabled } = args;
        Self {
            id, position, enabled, pressed,
            state: InteractState::None,
        }
    }

    #[must_use]
    pub fn hovered(&self) -> bool {
        match self.state {
            InteractState::Hovered | InteractState::Pressed => true,
            InteractState::None | InteractState::Disabled => false,
        }
    }

    pub fn update(&mut self, user_state:&UserState, game_state:&GameState, camera:&Camera) {
        self.state = if !(self.enabled)(game_state) {
            InteractState::Disabled
        } else {
            match (self.state, GridPoint::nearest(camera.to_world(user_state.mouse_position)) == self.position) {
                (InteractState::None | InteractState::Disabled, true) => InteractState::Hovered,
                (_, false) => InteractState::None,
                (state, _) => state,
            }
        }
    }

    pub fn mouse_input(
        &mut self, button_state:ElementState, mouse_button:MouseButton,
        user_state:&UserState, camera:&Camera, game_state:&mut GameState
    ) {
        if !matches!(mouse_button, MouseButton::Left) { return; };
        match (self.state, button_state) {
            (InteractState::Hovered, ElementState::Pressed) => self.state = InteractState::Pressed,
            (InteractState::Pressed, ElementState::Released) => {
                (self.pressed)(game_state);
                self.state = InteractState::None;
                self.update(user_state, game_state, camera);
            },
            _ => {},
        }
    }
}

