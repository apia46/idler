use crate::game::GameState;
use crate::grid::GridPoint;
use crate::input::button::{Button as InputButton, NewButtonArgs as NewInputButtonArgs};
use crate::render::Renderer;
use crate::render::{Renderable, button::{Button as RenderButton, NewButtonArgs as NewRenderButtonArgs}};
use crate::state::State;

#[derive(Clone, Copy)]
pub struct NewButtonArgs {
    pub position: GridPoint,
    pub text: fn(&GameState) -> String,
    pub enabled: fn(&GameState) -> bool,
    pub pressed: fn(&mut GameState),
}

impl State {
    #[inline(always)]
    pub fn new_button(&mut self, args:NewButtonArgs, renderer:&mut Renderer) {
        let NewButtonArgs { position, text, enabled, pressed } = args;
        let count = &mut self.entities.buttons;
        self.input_state.buttons.push(InputButton::new(*count, NewInputButtonArgs { position, enabled, pressed }));
        self.render_state.buttons.push(RenderButton::new(*count, NewRenderButtonArgs { position, text }, renderer, &self.game_state));
        *count += 1;
    }
}

