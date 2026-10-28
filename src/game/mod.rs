use std::time::Duration;

use educe::Educe;
use vello::kurbo::{Point, Size};

use crate::{num::{AddBoost, MulBoost, Number, PowBoost, Quantity}, render::Renderer, state::{State, button::NewButtonArgs, label::NewLabelArgs}};

#[derive(Educe)]
#[educe(Default)]
pub struct GameState {
    #[educe(Default=Quantity::ONE)]
    pub things: Quantity,
    pub things_per_second: AddBoost,
    pub things_mult: MulBoost,
    pub things_exp: PowBoost,
}

impl GameState {
    pub fn update(&mut self, dt:Duration) {
        self.things += (self.things_per_second * self.things_mult ^ self.things_exp) * dt;
    }
}

pub fn test_section(state:&mut State, renderer:&mut Renderer) {
    state.new_label(NewLabelArgs {
        position: Point::new(20., 20.),
        text: |s| format!("Things: {}\n+({} x {}) ^ {} = {}/s", s.things, s.things_per_second, s.things_mult, s.things_exp,
            (s.things_per_second * s.things_mult) ^ s.things_exp
        ),
    }, renderer);
    state.new_button(NewButtonArgs {
        position: Point::new(40., 120.),
        size: Size::new(160., 100.),
        text: |s| format!("Convert things into\n+{}\nthings per second", s.things),
        enabled: |s| s.things > Number::ZERO,
        pressed: |s| {
            s.things_per_second += s.things.to_number();
            s.things = Quantity::ZERO;
        },
    }, renderer);
    state.new_button({
        fn expr(s:&GameState) -> Number { s.things_per_second.to_number().ln() }
        NewButtonArgs {
            position: Point::new(240., 120.),
            size: Size::new(200., 100.),
            text: |s| format!("Reset things and convert\nthings per second into\n+{}\nthings mult", expr(s)),
            enabled: |s| expr(s) > Number::ZERO,
            pressed: |s| {
                s.things_mult += expr(s);
                s.things_per_second = AddBoost::ZERO;
                s.things = Quantity::ONE;
            },
        }
    }, renderer);
    state.new_button({
        fn expr(s:&GameState) -> Number { s.things_mult.to_number().ln().ln() - s.things_exp.to_number() }
        NewButtonArgs {
            position: Point::new(40., 260.),
            size: Size::new(200., 100.),
            text: |s| format!("Reset things and\nthings per second and\nconvert things mult into\n+{}\nthings exponent", expr(s)),
            enabled: |s| expr(s) > Number::ZERO,
            pressed: |s| {
                s.things_exp += expr(s);
                s.things_mult = MulBoost::ONE;
                s.things_per_second = AddBoost::ZERO;
                s.things = Quantity::ONE;
            },
        }
    }, renderer);
}

