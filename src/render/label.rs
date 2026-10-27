use vello::kurbo::Point;

use crate::{game::GameState, input::InputState, render::{BuildTextArgs, Renderable, Renderer, TextLayout}};

pub struct Label {
    #[expect(unused)]
    id: usize,
    position: Point,
    text_fn: fn(&GameState) -> String,
    layout: TextLayout,
}

pub struct NewLabelArgs {
    pub position: Point,
    pub text: fn(&GameState) -> String,
}

impl Renderable for Label {
    type NewArgs = NewLabelArgs;
     fn new(id:usize, args:NewLabelArgs, renderer:&mut Renderer, state:&GameState) -> Self {
        let NewLabelArgs { position, text:text_fn } = args;
        let text = &(args.text)(state);
        Label { id, position, text_fn, layout: renderer.build_text(BuildTextArgs { text }) }
    }

    fn update(&mut self, renderer:&mut Renderer, game_state:&GameState, _input_state:&InputState) {
        let text = &(self.text_fn)(game_state);
        self.layout = renderer.build_text(BuildTextArgs { text });
    }

    fn draw(&self, renderer:&mut Renderer) {
        renderer.draw_text(self.position.to_vec2(), &self.layout);
    }
}

