use std::time::{Duration, Instant};

use crate::num2::Number;

pub struct RenderState {
    pub start_time: Instant,
    pub previous_frame: Duration,
    pub things: Number,
    pub things_per_second: Number,
}

impl RenderState {
    pub fn new() -> Self {
        RenderState {
            start_time: Instant::now(),
            previous_frame: Duration::ZERO,
            things: 1.into(),
            things_per_second: 0.into(),
        }
    }
}

