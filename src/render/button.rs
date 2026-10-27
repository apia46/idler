use vello::kurbo::{Point, Size};

use crate::{game::GameState, render::{DrawRectArgs, DrawTextArgs, Renderable, Renderer}};

pub struct Button {
    position: Point,
    size: Size,
    text_fn: fn(&GameState) -> String,
    text: String,
}

pub struct NewButtonArgs {
    pub position: Point,
    pub size: Size,
    pub text: fn(&GameState) -> String,
}

impl Renderable for Button {
    type NewArgs = NewButtonArgs;
    fn new(args:NewButtonArgs, state:&GameState) -> Self {
        let NewButtonArgs { position, size, text:text_fn } = args;
        let text = (args.text)(state);
        Button { position, size, text_fn, text }
    }

    fn update(&mut self, state:&GameState) {
        self.text = (self.text_fn)(state);
    }

    fn draw(&self, renderer:&mut Renderer) {
        renderer.draw_rect(DrawRectArgs { rect: (self.position, self.size).into() });
        renderer.draw_text(DrawTextArgs { text: &self.text, position: self.position.to_vec2() })
    }
}

