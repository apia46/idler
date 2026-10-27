use vello::kurbo::Point;

use crate::game::GameState;
use crate::render::{Renderable, label::{Label as RenderLabel, NewLabelArgs as NewRenderLabelArgs}};
use crate::state::State;

pub struct NewLabelArgs {
    pub position: Point,
    pub text: fn(&GameState) -> String,
}

impl State {
    pub fn new_label(&mut self, args:NewLabelArgs) {
        let NewLabelArgs { position, text } = args;
        self.render_state.labels.push(RenderLabel::new(NewRenderLabelArgs { position, text }, &self.logic_state));
    }
}

