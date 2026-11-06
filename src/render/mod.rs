use std::time::Duration;

use macros::parse_color;
use parley::{FontContext, LayoutContext};
use vello::{Scene, peniko::{Color, Gradient, ImageBrush, ImageData}};

pub const BG_COLOR:Color = parse_color!("#224466");

use crate::game::GameState;
use crate::render::camera::Camera;
use crate::input::InputState;

mod state;
mod draw;
pub mod camera;

pub mod button;
pub mod label;
pub mod segment;

pub use state::RenderState;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorBrush(Color);

impl Default for ColorBrush {
    fn default() -> Self {
        ColorBrush(Color::BLACK)
    }
}

impl From<ColorBrush> for vello::peniko::Brush<ImageBrush<&ImageData>, &Gradient> {
    fn from(ColorBrush(value): ColorBrush) -> Self {
        value.into()
    }
}

#[derive(Default)]
pub struct Renderer {
    scene: Scene,
    font_cx: FontContext,
    layout_cx: LayoutContext<ColorBrush>,
}

impl Renderer {
    pub fn update(&mut self, render_state: &mut RenderState, game_state:&GameState, input_state:&InputState) {
        render_state.update(self, game_state, input_state);
    }

    pub fn process(&mut self, render_state: &mut RenderState, input_state:&InputState, dt:Duration) {
        render_state.camera.process(&input_state.user_state, dt);
    }

    pub fn draw_scene(&mut self, state:&RenderState) -> &Scene {
        self.scene.reset();
        state.draw(self);
        &self.scene
    }
}

pub trait Renderable {
    type NewArgs;
    fn new(id:usize, args:Self::NewArgs, renderer:&mut Renderer, state:&GameState) -> Self;

    fn update(&mut self, renderer:&mut Renderer, game_state:&GameState, input_state:&InputState) {
        let _ =renderer;
        let _ = game_state;
        let _ = input_state;
    }

    fn draw(&self, renderer:&mut Renderer, camera:&Camera);
}

