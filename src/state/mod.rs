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
    pub fn update(&mut self, renderer:&mut Renderer, dt:Duration) {
        self.game_state.update(dt);
        self.input_state.update(&self.game_state, &self.render_state.camera);
        renderer.update(&mut self.render_state, &self.game_state, &self.input_state);
    }

    pub fn process(&mut self, renderer:&mut Renderer, dt:Duration) {
        renderer.process(&mut self.render_state, &self.input_state, dt);
    }
}

