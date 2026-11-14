use macros::parse_color;
use vello::{kurbo::Point, peniko::Color};

use crate::game::GameState;
use crate::render::UpdateContext;
use crate::render::{camera::Camera, draw::{BuildTextArgs, TextLayout}, Renderable, Renderer};
use crate::state::label::LabelKey;

const FONT_SIZE:f32 = 16.;
const TEXT_COLOR:Color = parse_color!("#ffffff");

pub struct Label {
    #[expect(unused)]
    key: LabelKey,
    position: Point,
    text_fn: fn(&GameState) -> String,
    layout: TextLayout,
}

pub struct NewLabelArgs<'a> {
    pub key: LabelKey,
    pub position: Point,
    pub text: fn(&GameState) -> String,
    pub renderer:&'a mut Renderer,
    pub state:&'a GameState,
}

#[must_use]
fn build_text(renderer:&mut Renderer, text:&str) -> TextLayout {
    renderer.build_text(BuildTextArgs { text, font_size: FONT_SIZE, color: TEXT_COLOR, center: false })
}

impl<'a> Renderable<'a> for Label {
    type NewArgs = NewLabelArgs<'a>;
     fn new(args:NewLabelArgs) -> Self {
        let NewLabelArgs { key, position, text:text_fn, renderer, state } = args;
        let text = &(args.text)(state);
        Label { key, position, text_fn, layout: build_text(renderer, text) }
    }

    fn update(&mut self, ctx:&mut UpdateContext) {
        let text = &(self.text_fn)(ctx.game_state);
        self.layout = build_text(ctx.renderer, text);
    }

    fn draw(&self, renderer:&mut Renderer, camera:&Camera) {
        renderer.draw_text(self.position.to_vec2(), &self.layout, camera);
    }
}

