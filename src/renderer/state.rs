use std::time::{Duration, Instant};

pub struct RenderState {
    pub start_time: Instant,
    pub previous_frame: Duration,
}

impl RenderState {
    pub fn new() -> Self {
        RenderState {
            start_time: Instant::now(),
            previous_frame: Duration::ZERO,
        }
    }
}

