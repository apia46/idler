use std::time::Duration;

use educe::Educe;
use macros::parse_color;
use parley::{FontContext, LayoutContext};
use vello::{Scene, peniko::{Color, Gradient, ImageBrush, ImageData}};

pub const BG_COLOR:Color = parse_color!("#161e4e");

use crate::{game::GameState, hover::HoverState, window::Window};
use crate::render::camera::Camera;
use crate::input::InputState;

mod state;
pub mod draw;
pub mod camera;
pub mod grid;
mod components;

pub use state::RenderState;
pub use components::{button, hover, label, resource, upgrade};

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

#[derive(Educe)]
#[educe(Default)]
pub struct RendererState {
    pub previous_frame: Duration,
    pub delta_time: f64,
    #[educe(Default=Duration::from_secs_f64(1./60.))]
    pub refresh_rate: Duration,
}

impl Renderer {
    pub fn update(&mut self, render_state: &mut RenderState, game_state:&GameState, input_state:&InputState) {
        render_state.update(&mut UpdateContext { renderer: self, game_state, input_state });
    }

    pub fn process(&mut self, render_state: &mut RenderState, input_state:&InputState, delta_time:f64) {
        render_state.camera.process(&input_state.user_state, delta_time);
        render_state.process(&mut ProcessContext { delta_time });
    }

    #[must_use]
    pub fn draw_scene(&mut self, state:&RenderState, window:&Window, hover_state:&HoverState) -> &Scene {
        self.scene.reset();
        grid::draw(self, &state.camera, window);
        state.draw(self);
        hover_state.draw(self);
        &self.scene
    }
}

pub struct UpdateContext<'a> {
    renderer: &'a mut Renderer,
    game_state: &'a GameState,
    input_state: &'a InputState,
}

pub struct ProcessContext {
    delta_time: f64,
}

pub trait Renderable<'a> {
    type NewArgs;
    #[must_use]
    fn new(args:Self::NewArgs) -> Self;

    fn update(&mut self, ctx:&mut UpdateContext) {
        _ = ctx;
    }

    fn process(&mut self, ctx:&mut ProcessContext) {
        _ = ctx;
    }

    fn draw(&self, renderer:&mut Renderer, camera:&Camera);
}

