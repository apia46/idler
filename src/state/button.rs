use crate::game::GameState;
use crate::grid::GridPoint;
use crate::hover::HoverInfo;
use crate::input::button::{Button as InputButton, NewButtonArgs as NewInputButtonArgs};
use crate::render::{Renderer, Renderable, button::{Button as RenderButton, NewButtonArgs as NewRenderButtonArgs}};
use crate::state::State;

super::new_key!(ButtonKey);

#[derive(Clone, Copy)]
pub struct NewButtonArgs {
    pub position: GridPoint,
    pub text: fn(&GameState) -> String,
    pub hover: fn(&GameState) -> Box<[HoverInfo]>,
    pub enabled: fn(&GameState) -> bool,
    pub pressed: fn(&mut GameState),
}

impl State {
    pub fn new_button(&mut self, args:NewButtonArgs, renderer:&mut Renderer) {
        let NewButtonArgs { position, text, hover, enabled, pressed } = args;
        self.entities.buttons.add(|key| {
            self.input_state.buttons.insert(key, InputButton::new(NewInputButtonArgs { key, position, enabled, pressed }));
            self.render_state.buttons.insert(key, RenderButton::new(NewRenderButtonArgs { key, position, text, hover, renderer, state: &self.game_state }));
        });
    }
}

