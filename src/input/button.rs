use vello::kurbo::{Point, Rect, Size};

use crate::input::InputState;
use crate::game::GameState;

pub struct Button {
    pub position: Point,
    pub size: Size,
    pub pressed: fn(&mut GameState),
}

impl Button {
    pub fn hovered(&self, state:&InputState) -> bool {
        Rect::from((self.position, self.size)).contains((state.mouse_position.x, state.mouse_position.y))
    }
}

