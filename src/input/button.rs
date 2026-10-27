use vello::kurbo::{Point, Rect, Size};
use winit::event::{ElementState, MouseButton};

use crate::game::GameState;
use crate::input::state::UserState;

#[derive(Clone, Copy)]
pub enum InteractState { None, Hovered, Pressed }

pub struct Button {
    #[expect(unused)]
    id: usize,
    position: Point,
    size: Size,
    pressed: fn(&mut GameState),
    pub state: InteractState,
}

pub struct NewButtonArgs {
    pub position: Point,
    pub size: Size,
    pub pressed: fn(&mut GameState),
}

impl Button {
    pub fn new(id: usize, args: NewButtonArgs) -> Self {
        let NewButtonArgs { position, size, pressed } = args;
        Self {
            id, position, size, pressed,
            state: InteractState::None,
        }
    }

    fn hovered(&self, state:&UserState) -> bool {
        Rect::from((self.position, self.size)).contains((state.mouse_position.x, state.mouse_position.y))
    }

    pub fn update(&mut self, state:&UserState) {
        self.state = match (self.state, self.hovered(state)) {
            (InteractState::None, true) => InteractState::Hovered,
            (_, false) => InteractState::None,
            (state, _) => state,
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

