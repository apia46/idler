use std::time::Duration;

use crate::{game::GameState, render::{Renderable, Renderer, button::Button, label::Label}};

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
            pub fn update(&mut self, state:&GameState) {
                $(
                    for renderable in &mut self.$renderable_names {
                        Renderable::update(renderable, state);
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
    delta_time: f64;
    buttons: Vec<Button>,
    labels: Vec<Label>,
}

