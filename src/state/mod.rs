use std::time::{Duration, Instant};

use educe::Educe;

use crate::{game::GameState, hover::{self, HoverInfo}, input::{self, InputState}, render::{RenderState, Renderer}};

pub mod button;
pub mod label;
pub mod resource;
pub mod upgrade;

#[derive(Educe)]
#[educe(Default)]
pub struct State {
    #[educe(Default=Instant::now())]
    pub start_time: Instant,
    pub previous_tick: Duration,
    pub entities: Entities,
    pub input_state: InputState,
    pub render_state: RenderState,
    pub hover_infos: Vec<HoverInfo>,
    pub game_state: GameState,
}

#[derive(Default)]
pub struct Entities {
    pub buttons: keys::ButtonKey,
    #[expect(unused)]
    pub labels: keys::LabelKey,
    pub resources: keys::ResourceKey,
    pub upgrades: keys::UpgradeKey,
}

impl State {
    pub fn update(&mut self, renderer:&mut Renderer, delta_time:f64) {
        self.game_state.update(delta_time);
        self.input_state.update(&self.game_state);
        hover::update(self);
        renderer.update(&mut self.render_state, &self.game_state, &self.input_state);
    }

    pub fn process(&mut self, renderer:&mut Renderer, delta_time:f64) {
        renderer.process(&mut self.render_state, &self.input_state, delta_time);
        if self.render_state.camera.camera_moved {
            input::mouse_moved(self.input_state.user_state.mouse_position, self);
        }
    }
}

macro_rules! new_key {
    ($name:ident) => {
        #[derive(PartialEq, Eq, Hash, Clone, Copy, Default)]
        pub struct $name(usize);

        impl $name {
            #[allow(unused)]
            pub fn add(&mut self, func:impl FnOnce($name)) {
                func(*self);
                self.0 += 1;
            }
        }
    };
}
use new_key;

pub mod keys {
    pub use crate::state::{button::ButtonKey, label::LabelKey, resource::ResourceKey,  upgrade::UpgradeKey};
}

