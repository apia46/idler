use winit::event::{ElementState, MouseButton};

use crate::{game::GameState, input::{Inputable, MouseInputResponse, UserState, button::Button, upgrade::Upgrade}, render::camera::Camera};

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
                    for inputable in &mut self.$name {
                        Inputable::update(inputable, &self.user_state, state, camera);
                    }
                )*
            }

            pub fn mouse_input(&mut self, button_state:ElementState, mouse_button:MouseButton, camera:&Camera, game_state:&mut GameState) -> MouseInputResponse {
                let mut response = MouseInputResponse::default();
                $(
                    for inputable in &mut self.$name {
                        response |= inputable.mouse_input(button_state, mouse_button, &self.user_state, camera, game_state);
                        if response.consumed { break; }
                    }
                )*
                response
            }
        }
    }
}

input_state! {
    buttons: Vec<Button>,
    upgrades: Vec<Upgrade>,
}

