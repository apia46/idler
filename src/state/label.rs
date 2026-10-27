use vello::kurbo::Point;

use crate::game::GameState;
use crate::render::Renderer;
use crate::render::{Renderable, label::{Label as RenderLabel, NewLabelArgs as NewRenderLabelArgs}};
use crate::state::State;

pub struct NewLabelArgs {
    pub position: Point,
    pub text: fn(&GameState) -> String,
}

impl State {
    pub fn new_label(&mut self, args:NewLabelArgs, renderer:&mut Renderer) {
        let NewLabelArgs { position, text } = args;
        let count = &mut self.entities.labels;
        self.render_state.labels.push(RenderLabel::new(*count, NewRenderLabelArgs { position, text }, renderer, &self.game_state));
        *count += 1;
    }
}

