use std::time::Duration;

use educe::Educe;
use macros::parse_color;

use crate::grid::{Direction, GridPoint};
use crate::num::{AddBoost, DiscreteBoost, Quantity};
use crate::render::Renderer;
use crate::state::resource::{NewSegmentArgs, NewUpgradeArgs};
use crate::state::{State, button::NewButtonArgs, resource::{NewResourceArgs, ResourceBuilder}};

mod upgrade;

pub use upgrade::Upgrade;

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
    pub first_upgrade: Upgrade,
    pub second_upgrade: Upgrade,
}

impl GameState {
    pub fn update(&mut self, dt:Duration) {
        self.things += self.things_per_second * dt;
    }

    pub fn fetch<T: Copy>(&self, reference:Reference<T>) -> T {
        *(reference.0)(self)
    }

    pub fn fetch_mut<T>(&mut self, reference:Reference<T>) -> &mut T {
        (reference.1)(self)
    }
}

pub fn test_section(state:&mut State, renderer:&mut Renderer) {
    ResourceBuilder::new(NewResourceArgs {
        reference: reference!(things),
        start_button: NewButtonArgs {
            position: GridPoint::new(2, 2),
            text: |s| format!("Things: {}\n+1", s.things),
            enabled: |_| true,
            pressed: |s| s.things += DiscreteBoost::from(
                AddBoost::ONE
                + AddBoost::upgr(s.first_upgrade, 1.into())
                + AddBoost::upgr(s.second_upgrade, 2.into())
            ),
        },
        start_direction: Direction::UP,
        fill_color: parse_color!("#ffcc11"),
        back_color: parse_color!("#331166"),
        border_color: parse_color!("#859dad"),
    })
        .segment(NewSegmentArgs { direction: Direction::UP, length: 2, growth: 8.into() })
        .upgrade(NewUpgradeArgs { reference: reference!(first_upgrade), direction: Direction::RUP, description: "+1 thing per click".into() })
        .segment(NewSegmentArgs { direction: Direction::UP, length: 1, growth: 2.into() })
        .branch(|b| b
            .segment(NewSegmentArgs { direction: Direction::LUP, length: 1, growth: 2.into() })
            .segment(NewSegmentArgs { direction: Direction::UP, length: 1, growth: 2.into() })
            .upgrade(NewUpgradeArgs { reference: reference!(second_upgrade), direction: Direction::UP, description: "+2 things per click".into() })
        )
        .segment(NewSegmentArgs { direction: Direction::UP, length: 1, growth: 2.into() })
        .segment(NewSegmentArgs { direction: Direction::RUP, length: 3, growth: 8.into() })
    .build(state, renderer);
}

