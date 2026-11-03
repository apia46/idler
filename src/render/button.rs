use vello::peniko::Color;

use crate::game::GameState;
use crate::grid::GridPoint;
use crate::input::{InputState, button::InteractState};
use crate::render::draw::{DrawHexagonArgs, Style};
use crate::render::{camera::Camera, draw::{BuildTextArgs, TextLayout}, Renderable, Renderer};

const FONT_SIZE:f32 = 12.;
const TEXT_COLOR:Color = Color::WHITE;
const BG_COLOR:Color = Color::BLACK;
const HOVERED_COLOR:Color = Color::from_rgb8(100, 100, 100);
const PRESSED_COLOR:Color = Color::from_rgb8(50, 50, 50);
const DISABLED_COLOR:Color = Color::from_rgb8(50, 50, 50);
const DISABLED_TEXT_COLOR:Color = Color::from_rgb8(100, 100, 100);
const BORDER_COLOR:Color = Color::from_rgb8(120, 120, 120);

pub struct Button {
    id: usize,
    position: GridPoint,
    text_fn: fn(&GameState) -> String,
    layout: TextLayout,
    bg_color: Color,
}

pub struct NewButtonArgs {
    pub position: GridPoint,
    pub text: fn(&GameState) -> String,
}

fn build_text(renderer:&mut Renderer, text:&str, state:InteractState) -> TextLayout {
    renderer.build_text(BuildTextArgs {
        text, font_size: FONT_SIZE,
        color: if let InteractState::Disabled = state { DISABLED_TEXT_COLOR } else { TEXT_COLOR },
        center: true,
    })
}

impl Renderable for Button {
    type NewArgs = NewButtonArgs;
    fn new(id:usize, args:NewButtonArgs, renderer:&mut Renderer, state:&GameState) -> Self {
        let NewButtonArgs { position, text:text_fn } = args;
        let text = &(args.text)(state);
        Button {
            id, position, text_fn,
            layout: build_text(renderer, text, InteractState::None),
            bg_color: BG_COLOR,
        }
    }

    fn update(&mut self, renderer:&mut Renderer, game_state:&GameState, input_state:&InputState) {
        let text = &(self.text_fn)(game_state);
        let interact_state = input_state.buttons[self.id].state;
        self.bg_color = match interact_state {
            InteractState::None => BG_COLOR,
            InteractState::Hovered => HOVERED_COLOR,
            InteractState::Pressed => PRESSED_COLOR,
            InteractState::Disabled => DISABLED_COLOR,
        };
        self.layout = build_text(renderer, text, interact_state);
    }

    fn draw(&self, renderer:&mut Renderer, camera:&Camera) {
        let style = Style {
            stroke: Some((4., BORDER_COLOR)), fill: Some(self.bg_color)
        };
        renderer.draw_hexagon(DrawHexagonArgs { point: self.position, style }, camera);
        renderer.draw_text(self.position.to_point().to_vec2(), &self.layout, camera)
    }
}

