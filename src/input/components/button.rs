use winit::event::{ElementState, MouseButton};

use crate::game::GameState;
use crate::grid::GridPoint;
use crate::input::{Inputable, MouseInputContext, MouseInputResponse, UpdateContext, UserState};
use crate::state::button::ButtonKey;

#[derive(Clone, Copy)]
pub enum InteractState { Disabled, None, Hovered, Pressed }

pub struct Button {
    #[expect(unused)]
    key: ButtonKey,
    position: GridPoint,
    state: InteractState,
    enabled: fn(&GameState) -> bool,
    pressed: fn(&mut GameState),
}

impl Button {
    pub fn state(&self) -> InteractState {
        self.state
    }
}

pub struct NewButtonArgs {
    pub key: ButtonKey,
    pub position: GridPoint,
    pub enabled: fn(&GameState) -> bool,
    pub pressed: fn(&mut GameState),
}

pub fn next_state(current_state:InteractState, position:GridPoint, user_state:&UserState, disable:bool) -> InteractState {
    if disable {
        InteractState::Disabled
    } else {
        match (current_state, user_state.mouse_grid_point == position) {
            (InteractState::None | InteractState::Disabled, true) => InteractState::Hovered,
            (_, false) => InteractState::None,
            (state, _) => state,
        }
    }
}

impl Inputable for Button {
    type NewArgs = NewButtonArgs;
    fn new(args:NewButtonArgs) -> Self {
        let NewButtonArgs { key, position, pressed, enabled } = args;
        Self {
            key, position, enabled, pressed,
            state: InteractState::None,
        }
    }

    fn update(&mut self, ctx:&mut UpdateContext) {
        self.state = next_state(self.state, self.position, ctx.user_state, !(self.enabled)(ctx.game_state))
    }

    fn mouse_input(&mut self, ctx:&mut MouseInputContext) -> MouseInputResponse {
        let hovered = matches!(self.state, InteractState::Hovered | InteractState::Pressed);
        if !matches!(ctx.mouse_button, MouseButton::Left) { return MouseInputResponse { hovered, consumed: false }; };
        let consumed = match (self.state, ctx.button_state) {
            (InteractState::Hovered, ElementState::Pressed) => {
                self.state = InteractState::Pressed; true
            }
            (InteractState::Pressed, ElementState::Released) => {
                (self.pressed)(ctx.game_state);
                self.state = InteractState::None;
                self.state = next_state(self.state, self.position, ctx.user_state, !(self.enabled)(ctx.game_state));
                true
            },
            _ => false,
        };
        MouseInputResponse { hovered, consumed }
    }
}

