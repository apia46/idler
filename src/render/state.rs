use std::time::Duration;

use crate::game::GameState;
use crate::input::InputState;
use crate::render::{camera::Camera, Renderable, Renderer, button::Button, label::Label};

macro_rules! render_state {
    {
        $($names:ident: $types:ty,)*
    } => {
        #[derive(Default)]
        pub struct RenderState {
            pub renderer_state: RendererState,
            pub camera: Camera,
            $(pub $names: $types,)*
        }

        impl RenderState {
            pub fn update(&mut self, renderer:&mut Renderer, game_state:&GameState, input_state:&InputState) {
                $(
                    for renderable in &mut self.$names {
                        Renderable::update(renderable, renderer, game_state, input_state);
                    }
                )*
            }

            pub fn draw(&self, renderer:&mut Renderer) {
                $(
                    for renderable in &self.$names {
                        Renderable::draw(renderable, renderer, &self.camera);
                    }
                )*
            }
        }
    }
}

#[derive(Default)]
pub struct RendererState {
    pub previous_frame: Duration,
    pub delta_time: Duration,
}

render_state! {
    buttons: Vec<Button>,
    labels: Vec<Label>,
}

