use std::time::{Duration, Instant};

use crate::{input::InputState, num::Number, render::RenderState};

pub struct State {
    pub start_time: Instant,
    pub previous_tick: Duration,
    pub things: Number,
    pub things_per_second: Number,
    pub things_mult: Number,
    pub input_state: InputState,
    pub render_state: RenderState,
}

impl Default for State {
    fn default() -> Self {
        State {
            start_time: Instant::now(),
            previous_tick: Duration::ZERO,
            things: 1.into(),
            things_per_second: 0.into(),
            things_mult: 1.into(),
            input_state: InputState::default(),
            render_state: RenderState::default(),
        }
    }
}

impl State {
    pub fn update<const UPDATE:bool>(&mut self, dt:f64) {
        let incr = self.things_per_second * self.things_mult * dt.into();
        if incr != Number::ZERO || UPDATE {
            self.things += incr;
            self.render_state.text = format!("things: {}\nthings per second: {}\nthings mult: {}\nclick to convert things to things per second\nspace to convert ln(things per second) to things mult", self.things, self.things_per_second, self.things_mult);
        }
    }
}

