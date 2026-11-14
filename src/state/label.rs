use vello::kurbo::Point;

use crate::game::GameState;
use crate::render::Renderer;
use crate::render::{Renderable, label::{Label as RenderLabel, NewLabelArgs as NewRenderLabelArgs}};
use crate::state::State;

super::new_key!(LabelKey);

pub struct NewLabelArgs {
    pub position: Point,
    pub text: fn(&GameState) -> String,
}

impl State {
    #[expect(unused)]
    pub fn new_label(&mut self, args:NewLabelArgs, renderer:&mut Renderer) {
        let NewLabelArgs { position, text } = args;
        self.entities.labels.add(|key| {
            self.render_state.labels.insert(key, RenderLabel::new( NewRenderLabelArgs { key, position, text, renderer, state: &self.game_state}));
        });
    }
}

