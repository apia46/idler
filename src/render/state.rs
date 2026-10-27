use std::time::Duration;
use educe::Educe;

#[derive(Educe)]
#[educe(Default)]
pub struct RenderState {
    pub previous_frame: Duration,
    pub delta_time: f64,
    pub text: String,
}

