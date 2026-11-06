use std::time::Duration;

use educe::Educe;
use macros::parse_color;

use crate::grid::{Direction, GridPoint};
use crate::num::{AddBoost, Number, Quantity};
use crate::render::Renderer;
use crate::state::{State, button::NewButtonArgs, resource::{NewResourceArgs, NewSegmentArgs}};

pub type Reference<T> = (fn(&GameState) -> &T, fn(&mut GameState) -> &mut T);

macro_rules! reference {
    ($resource:ident) => {
        (|s| &s.$resource, |s| &mut s.$resource)
    };
}

#[derive(Educe)]
#[educe(Default)]
pub struct GameState {
    #[educe(Default=0)]
    pub things: Quantity,
    pub things_per_second: AddBoost,
}

impl GameState {
    pub fn update(&mut self, dt:Duration) {
        self.things += self.things_per_second * dt;
    }
}

pub fn test_section(state:&mut State, renderer:&mut Renderer) {
    state.new_resource(NewResourceArgs {
        reference: reference!(things),
        start_button: NewButtonArgs {
            position: GridPoint::new(2, 2), text: |s| format!("Things: {}\n+1", s.things),
            enabled: |_| true,
            pressed:|s| s.things += Number::ONE,
        },
        fill_color: parse_color!("#ffcc11"),
        back_color: parse_color!("#331166"),
        border_color: parse_color!("#859dad"),
        segments: vec![
            NewSegmentArgs { direction: Direction::UP, length: 4, growth: Number::new(16.) },
            NewSegmentArgs { direction: Direction::RUP, length: 2, growth: Number::new(4.) },
        ],
    }, renderer);
}

