use vello::kurbo::{Point, Rect, Size};
use winit::event::{ElementState, MouseButton};

use crate::game::GameState;
use crate::input::state::UserState;

#[derive(Clone, Copy)]
pub enum InteractState { Disabled, None, Hovered, Pressed }

pub struct Button {
    #[expect(unused)]
    id: usize,
    position: Point,
    size: Size,
    enabled: fn(&GameState) -> bool,
    pressed: fn(&mut GameState),
    pub state: InteractState,
}

pub struct NewButtonArgs {
    pub position: Point,
    pub size: Size,
    pub enabled: fn(&GameState) -> bool,
    pub pressed: fn(&mut GameState),
}

impl Button {
    pub fn new(id: usize, args: NewButtonArgs) -> Self {
        let NewButtonArgs { position, size, pressed, enabled } = args;
        Self {
            id, position, size, enabled, pressed,
            state: InteractState::None,
        }
    }

    fn hovered(&self, state:&UserState) -> bool {
        Rect::from((self.position, self.size)).contains((state.mouse_position.x, state.mouse_position.y))
    }

    pub fn update(&mut self, user_state:&UserState, game_state:&GameState) {
        self.state = if !(self.enabled)(game_state) {
            InteractState::Disabled
        } else {
            match (self.state, self.hovered(user_state)) {
                (InteractState::None | InteractState::Disabled, true) => InteractState::Hovered,
                (_, false) => InteractState::None,
                (state, _) => state,
            }
        }
    }
    
    pub fn mouse_input(&mut self, button_state:ElementState, mouse_button:MouseButton, game_state:&mut GameState) {
        if !matches!(mouse_button, MouseButton::Left) { return; };
        self.state = match (self.state, button_state) {
            (InteractState::Hovered, ElementState::Pressed) => InteractState::Pressed,
            (InteractState::Pressed, ElementState::Released) => {
                (self.pressed)(game_state);
                InteractState::None
            },
            (state, _) => state,
        }
    }
}

