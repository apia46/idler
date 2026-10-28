use vello::{kurbo::Point, peniko::Color};

use crate::game::GameState;
use crate::input::InputState;
use crate::render::{camera::Camera, BuildTextArgs, Renderable, Renderer, TextLayout};

const TEXT_COLOR:Color = Color::WHITE;

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
        Label { id, position, text_fn, layout: renderer.build_text(BuildTextArgs { text, color: TEXT_COLOR }) }
    }

    fn update(&mut self, renderer:&mut Renderer, game_state:&GameState, _input_state:&InputState) {
        let text = &(self.text_fn)(game_state);
        self.layout = renderer.build_text(BuildTextArgs { text, color: TEXT_COLOR });
    }

    fn draw(&self, renderer:&mut Renderer, camera:&Camera) {
        renderer.draw_text(self.position.to_vec2(), &self.layout, camera);
    }
}

