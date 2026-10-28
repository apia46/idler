use std::time::Duration;

use crate::game::GameState;
use crate::input::InputState;
use crate::render::{Renderable, Renderer, button::Button, label::Label};

macro_rules! render_state {
    {
        $($names:ident: $types:ty),*;
        $($renderable_names:ident: $renderable_types:ty,)*
    } => {
        #[derive(Default)]
        pub struct RenderState {
            $(pub $names: $types,)*
            $(pub $renderable_names: $renderable_types,)*
        }

        impl RenderState {
            pub fn update(&mut self, renderer:&mut Renderer,game_state:&GameState, input_state:&InputState) {
                $(
                    for renderable in &mut self.$renderable_names {
                        Renderable::update(renderable, renderer, game_state, input_state);
                    }
                )*
            }

            pub fn draw(&self, renderer:&mut Renderer) {
                $(
                    for renderable in &self.$renderable_names {
                        Renderable::draw(renderable, renderer);
                    }
                )*
            }
        }
    }
}

render_state! {
    previous_frame: Duration,
    delta_time: Duration;
    buttons: Vec<Button>,
    labels: Vec<Label>,
}

