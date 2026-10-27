use std::time::{Duration, Instant};

use educe::Educe;

use crate::{game::GameState, input::InputState, render::{RenderState, Renderer}};

pub mod button;
pub mod label;

#[derive(Educe)]
#[educe(Default)]
pub struct State {
    #[educe(Default=Instant::now())]
    pub start_time: Instant,
    pub previous_tick: Duration,
    pub entities: Entities,
    pub input_state: InputState,
    pub render_state: RenderState,
    pub game_state: GameState,
}

#[derive(Default)]
pub struct Entities {
    pub buttons: usize,
    pub labels: usize,
}

impl State {
    pub fn update(&mut self,renderer:&mut Renderer, dt:f64) {
        self.game_state.update(dt.into());
        self.input_state.update();
        self.render_state.update(renderer, &self.game_state, &self.input_state);
    }
}

