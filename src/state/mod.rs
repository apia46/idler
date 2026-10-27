use std::time::{Duration, Instant};

use educe::Educe;

use crate::{input::InputState, game::GameState, render::RenderState};

pub mod button;

#[derive(Educe)]
#[educe(Default)]
pub struct State {
    #[educe(Default=Instant::now())]
    pub start_time: Instant,
    pub previous_tick: Duration,
    pub input_state: InputState,
    pub render_state: RenderState,
    pub logic_state: GameState,
}

impl State {
    pub fn update(&mut self, dt:f64) {
        self.logic_state.update(dt.into());
        self.render_state.update(&self.logic_state);
    }
}

