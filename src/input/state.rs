use winit::dpi::PhysicalPosition;

use crate::input::button::Button;

#[derive(Default)]
pub struct InputState {
    pub mouse_position: PhysicalPosition<f64>,
    pub buttons: Vec<Button>,
}

