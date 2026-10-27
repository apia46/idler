use vello::kurbo::Point;

use crate::{game::GameState, render::{DrawTextArgs, Renderable, Renderer}};

pub struct Label {
    position: Point,
    text_fn: fn(&GameState) -> String,
    text: String,
}

pub struct NewLabelArgs {
    pub position: Point,
    pub text: fn(&GameState) -> String,
}

impl Renderable for Label {
    type NewArgs = NewLabelArgs;
     fn new(args:NewLabelArgs, state:&GameState) -> Self {
        let NewLabelArgs { position, text:text_fn } = args;
        let text = (args.text)(state);
        Label { position, text_fn, text }
    }

    fn update(&mut self, state:&GameState) {
        self.text = (self.text_fn)(state);
    }

    fn draw(&self, renderer:&mut Renderer) {
        renderer.draw_text(DrawTextArgs { text: &self.text, position: self.position.to_vec2() })
    }
}

