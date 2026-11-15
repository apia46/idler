use std::collections::HashMap;

use winit::event::{ElementState, MouseButton};

use crate::{game::GameState, input::{Inputable, MouseInputResponse, UserState, UpdateContext, MouseInputContext, HoverContext, button::Button, upgrade::Upgrade, resource::ResourceTree}};
use crate::state::keys::*;

macro_rules! input_state {
    {
        $($name:ident: $type:ty,)*
    } => {
        #[derive(Default)]
        pub struct InputState {
            pub user_state: UserState,
            $(pub $name: $type,)*
        }

        impl InputState {
            pub fn update(&mut self, state:&GameState) {
                $(
                    for inputable in self.$name.values_mut() {
                        Inputable::update(inputable, &mut UpdateContext {
                            game_state: state,
                            user_state: &self.user_state,
                        });
                    }
                )*
            }

            #[must_use]
            pub fn mouse_input(&mut self, button_state:ElementState, mouse_button:MouseButton, game_state:&mut GameState) -> MouseInputResponse {
                let mut response = MouseInputResponse::default();
                $(
                    for inputable in self.$name.values_mut() {
                        response |= inputable.mouse_input(&mut MouseInputContext {
                            button_state, mouse_button, user_state: &self.user_state, game_state
                        });
                        if response.consumed { break; }
                    }
                )*
                response
            }

            #[must_use]
            pub fn get_hovers(&self) -> Vec<Hover> {
                let mut hovers = Vec::new();
                $(
                    for inputable in self.$name.values() {
                        if let Some(key) = Inputable::hovered(inputable, &mut HoverContext {
                            user_state: &self.user_state
                        }) {
                            hovers.push(key);
                        }
                    }
                )*
                hovers
            }
        }
    }
}

input_state! {
    resource_trees: HashMap<ResourceKey, ResourceTree>,
    buttons: HashMap<ButtonKey, Button>,
    upgrades: HashMap<UpgradeKey, Upgrade>,
}

pub enum Hover {
    ResourceTree(ResourceKey, usize),
}

