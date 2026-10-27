use vello::kurbo::{Point, Size};

use crate::game::GameState;
use crate::input::button::Button as InputButton;
use crate::render::{Renderable, button::{Button as RenderButton, NewButtonArgs as NewRenderButtonArgs}};
use crate::state::State;

pub struct NewButtonArgs {
    pub position: Point,
    pub size: Size,
    pub text: fn(&GameState) -> String,
    pub pressed: fn(&mut GameState),
}

impl State {
    pub fn new_button(&mut self, args:NewButtonArgs) {
        let NewButtonArgs { position, size, text, pressed } = args;
        self.input_state.buttons.push(InputButton { position, size, pressed });
        self.render_state.buttons.push(RenderButton::new(NewRenderButtonArgs { position, size, text }, &self.logic_state));
    }
}

