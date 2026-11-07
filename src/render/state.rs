use crate::game::GameState;
use crate::input::InputState;
use crate::render::{RendererState, camera::Camera, Renderable, Renderer, button::Button, label::Label, upgrade::Upgrade, resource::ResourceTree};

macro_rules! render_state {
    {
        $($name:ident: $type:ty,)*
    } => {
        #[derive(Default)]
        pub struct RenderState {
            pub renderer_state: RendererState,
            pub camera: Camera,
            $(pub $name: $type,)*
        }

        impl RenderState {
            pub fn update(&mut self, renderer:&mut Renderer, game_state:&GameState, input_state:&InputState) {
                $(
                    for renderable in &mut self.$name {
                        Renderable::update(renderable, renderer, game_state, input_state);
                    }
                )*
            }

            pub fn draw(&self, renderer:&mut Renderer) {
                $(
                    for renderable in &self.$name {
                        Renderable::draw(renderable, renderer, &self.camera);
                    }
                )*
            }
        }
    }
}

render_state! {
    resource_trees: Vec<ResourceTree>,
    buttons: Vec<Button>,
    upgrades: Vec<Upgrade>,
    labels: Vec<Label>,
}

