use macros::parse_color;
use vello::peniko::Color;

use crate::game::GameState;
use crate::grid::GridPoint;
use crate::input::button::InteractState;
use crate::render::UpdateContext;
use crate::render::draw::{DrawHexagonArgs, Style};
use crate::render::{camera::Camera, draw::{BuildTextArgs, TextLayout}, Renderable, Renderer};
use crate::state::button::ButtonKey;

const FONT_SIZE:f32 = 12.;
const TEXT_COLOR:Color = parse_color!("#ffffff");
const BG_COLOR:Color = parse_color!("#666666");
const HOVERED_COLOR:Color = parse_color!("#808080");
const PRESSED_COLOR:Color = parse_color!("#555555");
const DISABLED_COLOR:Color = parse_color!("#333333");
const DISABLED_TEXT_COLOR:Color = parse_color!("#999999");
const BORDER_COLOR:Color = parse_color!("#444444");

pub struct Button {
    key: ButtonKey,
    position: GridPoint,
    text_fn: fn(&GameState) -> String,
    layout: TextLayout,
    bg_color: Color,
}

pub struct NewButtonArgs<'a> {
    pub key: ButtonKey,
    pub position: GridPoint,
    pub text: fn(&GameState) -> String,
    pub renderer:&'a mut Renderer,
    pub state:&'a GameState,
}

#[must_use]
fn build_text(renderer:&mut Renderer, text:&str, state:InteractState) -> TextLayout {
    renderer.build_text(BuildTextArgs {
        text, font_size: FONT_SIZE,
        color: if let InteractState::Disabled = state { DISABLED_TEXT_COLOR } else { TEXT_COLOR },
        center: true,
    })
}

impl<'a> Renderable<'a> for Button {
    type NewArgs = NewButtonArgs<'a>;
    fn new(args:NewButtonArgs) -> Self {
        let NewButtonArgs { key, position, text:text_fn, renderer, state } = args;
        let text = &(args.text)(state);
        Button {
            key, position, text_fn,
            layout: build_text(renderer, text, InteractState::None),
            bg_color: BG_COLOR,
        }
    }

    fn update(&mut self, ctx:&mut UpdateContext) {
        let text = &(self.text_fn)(ctx.game_state);
        let interact_state = ctx.input_state.buttons[&self.key].state();
        self.bg_color = match interact_state {
            InteractState::None => BG_COLOR,
            InteractState::Hovered => HOVERED_COLOR,
            InteractState::Pressed => PRESSED_COLOR,
            InteractState::Disabled => DISABLED_COLOR,
        };
        self.layout = build_text(ctx.renderer, text, interact_state);
    }

    fn draw(&self, renderer:&mut Renderer, camera:&Camera) {
        let style = Style {
            stroke: Some((6., BORDER_COLOR)), fill: Some(self.bg_color)
        };
        renderer.draw_hexagon(DrawHexagonArgs { point: self.position, style }, camera);
        renderer.draw_text(self.position.to_point().to_vec2(), &self.layout, camera)
    }
}

