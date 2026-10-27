use vello::kurbo::{Point, Size};

use crate::{num::Number, state::{State, button::NewButtonArgs, label::NewLabelArgs}};

pub struct GameState {
    pub things: Number,
    pub things_per_second: Number,
    pub things_mult: Number,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            things: Number::ONE,
            things_per_second: Number::ZERO,
            things_mult: Number::ONE,
        }
    }
}

impl GameState {
    pub fn update(&mut self, dt:Number) {
        self.things += self.things_per_second * self.things_mult * dt;
    }
}

pub fn test_section(state:&mut State) {
    state.new_label(NewLabelArgs {
        position: Point::new(20., 20.),
        text: |s| format!("Things: {}\n(+{} x {}/s)", s.things, s.things_per_second, s.things_mult),
    });
    state.new_button(NewButtonArgs {
        position: Point::new(40., 120.),
        size: Size::new(160., 100.),
        text: |s| format!("Convert things into\n{}\nthings per second", s.things),
        pressed: |s| {
            s.things_per_second += s.things;
            s.things = Number::ZERO;
        },
    });
    state.new_button(NewButtonArgs {
        position: Point::new(240., 120.),
        size: Size::new(200., 100.),
        text: |s| format!("Reset things and convert\nthings per second into\n{:.3}\nthings mult", s.things_per_second.safe_ln()),
        pressed: |s| {
            s.things_mult += s.things_per_second.safe_ln().into();
            s.things_per_second = Number::ZERO;
            s.things = Number::ONE;
        },
    });
}

