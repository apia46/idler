use vello::kurbo::{Point, Size};

use crate::{num::Number, render::Renderer, state::{State, button::NewButtonArgs, label::NewLabelArgs}};

pub struct GameState {
    pub things: Number,
    pub things_per_second: Number,
    pub things_mult: Number,
    pub things_exp: Number,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            things: Number::ONE,
            things_per_second: Number::ZERO,
            things_mult: Number::ONE,
            things_exp: Number::ONE,
        }
    }
}

impl GameState {
    pub fn update(&mut self, dt:Number) {
        self.things += ((self.things_per_second * self.things_mult) ^ self.things_exp) * dt;
    }
}

pub fn test_section(state:&mut State, renderer:&mut Renderer) {
    state.new_label(NewLabelArgs {
        position: Point::new(20., 20.),
        text: |s| format!("Things: {}\n+(({} x {}) ^ {}) = {}/s", s.things, s.things_per_second, s.things_mult, s.things_exp,
            (s.things_per_second * s.things_mult) ^ s.things_exp
        ),
    }, renderer);
    state.new_button(NewButtonArgs {
        position: Point::new(40., 120.),
        size: Size::new(160., 100.),
        text: |s| format!("Convert things into\n{}\nthings per second", s.things),
        enabled: |s| s.things > Number::ZERO,
        pressed: |s| {
            s.things_per_second += s.things;
            s.things = Number::ZERO;
        },
    }, renderer);
    state.new_button({
        fn expr(s:&GameState) -> Number {
            s.things_per_second.safe_ln().into()
        }
        NewButtonArgs {
            position: Point::new(240., 120.),
            size: Size::new(200., 100.),
            text: |s| format!("Reset things and convert\nthings per second into\n{:.3}\nthings mult", expr(s)),
            enabled: |s| expr(s) > Number::ZERO,
            pressed: |s| {
                s.things_mult += expr(s);
                s.things_per_second = Number::ZERO;
                s.things = Number::ONE;
            },
        }
    }, renderer);
    state.new_button({
        fn expr(s:&GameState) -> Number {
            let number:Number = s.things_mult.safe_ln().into();
            Number::from(number.safe_ln()) - s.things_exp
        }
        NewButtonArgs {
            position: Point::new(40., 260.),
            size: Size::new(200., 100.),
            text: |s| format!("Reset things and\nthings per second and\nconvert things mult into\n{:.3}\nthings exponent", expr(s)),
            enabled: |s| expr(s) > Number::ZERO,
            pressed: |s| {
                s.things_exp += expr(s);
                s.things_mult = Number::ONE;
                s.things_per_second = Number::ZERO;
                s.things = Number::ONE;
            },
        }
    }, renderer);

}

