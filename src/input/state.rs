use std::collections::HashMap;

use winit::event::{ElementState, MouseButton};

use crate::{game::GameState, input::{Inputable, MouseInputResponse, UserState, UpdateContext, MouseInputContext, button::Button, upgrade::Upgrade}, render::camera::Camera};
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
            pub fn update(&mut self, state:&GameState, camera:&Camera) {
                $(
                    for inputable in self.$name.values_mut() {
                        Inputable::update(inputable, &mut UpdateContext {
                            game_state: state,
                            user_state: &self.user_state,
                            camera,
                        });
                    }
                )*
            }

            pub fn mouse_input(&mut self, button_state:ElementState, mouse_button:MouseButton, camera:&Camera, game_state:&mut GameState) -> MouseInputResponse {
                let mut response = MouseInputResponse::default();
                $(
                    for inputable in self.$name.values_mut() {
                        response |= inputable.mouse_input(&mut MouseInputContext {
                            button_state, mouse_button, user_state: &self.user_state, camera, game_state
                        });
                        if response.consumed { break; }
                    }
                )*
                response
            }
        }
    }
}

input_state! {
    buttons: HashMap<ButtonKey, Button>,
    upgrades: HashMap<UpgradeKey, Upgrade>,
}

