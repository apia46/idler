use vello::kurbo::{Point, Rect, Size};

use crate::input::{self, InputState};
use crate::state::State;

pub const BUTTONS:[Button; 1] = [
    Button {
        position: Point::new(0., 0.),
        size: Size::new(100., 100.),
        pressed: input::this_button,
    }
];

type Pressed = fn(&mut State);

pub struct Button {
    position: Point,
    size: Size,
    pub pressed: Pressed,
}

impl Button {
    pub fn hovered(&self, state:&InputState) -> bool {
        Rect::from((self.position, self.size)).contains((state.mouse_position.x, state.mouse_position.y))
    }
}

