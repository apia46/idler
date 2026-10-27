use vello::kurbo::{Point, Size};

use crate::game::GameState;
use crate::input::button::{Button as InputButton, NewButtonArgs as NewInputButtonArgs};
use crate::render::Renderer;
use crate::render::{Renderable, button::{Button as RenderButton, NewButtonArgs as NewRenderButtonArgs}};
use crate::state::State;

pub struct NewButtonArgs {
    pub position: Point,
    pub size: Size,
    pub text: fn(&GameState) -> String,
    pub pressed: fn(&mut GameState),
}

impl State {
    pub fn new_button(&mut self, args:NewButtonArgs, renderer:&mut Renderer) {
        let NewButtonArgs { position, size, text, pressed } = args;
        let count = &mut self.entities.buttons;
        self.input_state.buttons.push(InputButton::new(*count, NewInputButtonArgs { position, size, pressed }));
        self.render_state.buttons.push(RenderButton::new(*count, NewRenderButtonArgs { position, size, text }, renderer, &self.game_state));
        *count += 1;
    }
}

