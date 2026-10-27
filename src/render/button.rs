use vello::{kurbo::{Point, Size}, peniko::Color};

use crate::{game::GameState, input::{InputState, button::InteractState}, render::{BuildTextArgs, DrawRectArgs, Renderable, Renderer, TextLayout}};

const BG_COLOR:Color = Color::BLACK;
const HOVERED_COLOR:Color = Color::from_rgb8(100, 100, 100);
const PRESSED_COLOR:Color = Color::from_rgb8(50, 50, 50);

pub struct Button {
    id: usize,
    position: Point,
    size: Size,
    text_fn: fn(&GameState) -> String,
    layout: TextLayout,
    bg_color: Color,
}

pub struct NewButtonArgs {
    pub position: Point,
    pub size: Size,
    pub text: fn(&GameState) -> String,
}

impl Renderable for Button {
    type NewArgs = NewButtonArgs;
    fn new(id:usize, args:NewButtonArgs, renderer:&mut Renderer, state:&GameState) -> Self {
        let NewButtonArgs { position, size, text:text_fn } = args;
        let text = &(args.text)(state);
        Button {
            id, position, size, text_fn,
            layout: renderer.build_text(BuildTextArgs { text }),
            bg_color: BG_COLOR,
        }
    }

    fn update(&mut self, renderer:&mut Renderer, game_state:&GameState, input_state:&InputState) {
        let text = &(self.text_fn)(game_state);
        self.bg_color = match input_state.buttons[self.id].state {
            InteractState::None => BG_COLOR,
            InteractState::Hovered => HOVERED_COLOR,
            InteractState::Pressed => PRESSED_COLOR,
        };
        self.layout = renderer.build_text(BuildTextArgs { text });
    }

    fn draw(&self, renderer:&mut Renderer) {
        renderer.draw_rect(DrawRectArgs { rect: (self.position, self.size).into(), color: self.bg_color });
        renderer.draw_text(self.position.to_vec2(), &self.layout)
    }
}

