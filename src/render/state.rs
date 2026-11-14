use std::collections::HashMap;

use crate::grid::GridPoint;
use crate::render::{RendererState, camera::Camera, Renderable, Renderer, UpdateContext, ProcessContext};
use crate::render::{button::Button, label::Label, upgrade::Upgrade, resource::ResourceTree, hover::HoverTile};
use crate::state::keys::*;

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
            pub fn update(&mut self, ctx:&mut UpdateContext) {
                $(
                    for renderable in self.$name.values_mut() {
                        Renderable::update(renderable, ctx);
                    }
                )*
            }

            pub fn process(&mut self, ctx:&mut ProcessContext) {
                $(
                    for renderable in self.$name.values_mut() {
                        Renderable::process(renderable, ctx);
                    }
                )*
            }

            pub fn draw(&self, renderer:&mut Renderer) {
                $(
                    for renderable in self.$name.values() {
                        Renderable::draw(renderable, renderer, &self.camera);
                    }
                )*
            }
        }
    }
}

render_state! {
    resource_trees: HashMap<ResourceKey, ResourceTree>,
    buttons: HashMap<ButtonKey, Button>,
    upgrades: HashMap<UpgradeKey, Upgrade>,
    labels: HashMap<LabelKey, Label>,
    hover_tiles: HashMap<GridPoint, HoverTile>,
}

