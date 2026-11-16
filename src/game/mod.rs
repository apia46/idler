use educe::Educe;
use macros::parse_color;

use crate::grid::{Direction, GridPoint};
use crate::hover::HoverInfo;
use crate::num::{Cost, Gain, Number, Quantity, RateOfChange};
use crate::render::Renderer;
use crate::state::resource::{NewSegmentArgs, NewUpgradeArgs};
use crate::state::{State, button::NewButtonArgs, resource::{NewResourceArgs, ResourceBuilder}};

mod upgrade;

pub use upgrade::Upgrade;

pub type Reference<T> = (fn(&GameState) -> &T, fn(&mut GameState) -> &mut T);

macro_rules! reference {
    ($($resource:ident).*) => {
        (|s| &s$(.$resource)*, |s| &mut s$(.$resource)*)
    };
}

#[derive(Educe)]
#[educe(Default)]
pub struct GameState {
    pub upgrades: Upgrades,
    #[educe(Default=0)]
    pub things: Quantity,
    pub things_per_click: Gain,
    pub things_rate: RateOfChange,
    #[educe(Default=0)]
    pub boosters: Quantity,
    pub boosters_cost: Cost,
    #[educe(Default=1)]
    pub boosters_effect: Number,
}

#[derive(Educe)]
#[educe(Default)]
pub struct Upgrades {
    pub click_1: Upgrade,
    pub click_2: Upgrade,
    #[educe(Default=0)]
    pub click_2_boost: Number,
    pub rate_1: Upgrade,
    pub rate_2: Upgrade,
    pub gen_1: Upgrade,
    pub gen_2: Upgrade,
    pub pow_1: Upgrade,
}

impl GameState {
    pub fn update(&mut self, delta_time:f64) {
        self.upgrades.click_2_boost = self.things_rate.to_number() / 4.into();
        self.things += self.things_rate * delta_time;
        self.things_per_click = Gain::ONE
            .add_boost(self.upgrades.click_1, 1)
            .add_boost(self.upgrades.click_2, self.upgrades.click_2_boost)
            .mul_boost(self.upgrades.gen_1, 2)
            .mul_boost(self.upgrades.gen_2, 2)
            .mul_boost(self.boosters > 0, self.boosters_effect)
        ;
        self.things_rate = RateOfChange::ZERO
            .add_boost(self.upgrades.rate_1, 2)
            .mul_boost(self.upgrades.rate_2, 2)
            .mul_boost(self.upgrades.gen_1, 2)
            .mul_boost(self.upgrades.gen_2, 2)
            .mul_boost(self.boosters > 0, self.boosters_effect)
            .pow_boost(self.upgrades.pow_1, 2)
        ;
        self.boosters_cost = Cost::from(1024).mul_boost(true, Number::from(1.4) ^ self.boosters.to_number());
        self.boosters_effect = Number::from(1.2) ^ self.boosters.to_number();
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
        name: "things",
        start_button: NewButtonArgs {
            position: GridPoint::new(0, 0),
            text: |s| format!("Things: {}\n+{}\n+{}/s", s.things, s.things_per_click, s.things_rate),
            hover: |s| Box::new([
                crate::hover::resource_hover(s, reference!(things), "things"),
                HoverInfo::Body(format!("+{} on click", s.things_per_click))
            ]),
            enabled: |_| true,
            pressed: |s| s.things += s.things_per_click,
        },
        start_direction: Direction::UP,
        growth: 2.into(),
        fill_color: parse_color!("#ffcc11"),
        back_color: parse_color!("#331166"),
        border_color: parse_color!("#859dad"),
    })
        .segment(NewSegmentArgs { direction: Direction::UP, length: 2 })
        .branch(|s| s
            .segment(NewSegmentArgs { direction: Direction::LUP, length: 2 })
            .upgrade(NewUpgradeArgs { reference: reference!(upgrades.click_1), direction: Direction::LOWN, description: |_| format!("+1 things per click") })
            .segment(NewSegmentArgs { direction: Direction::UP, length: 2 })
            .segment(NewSegmentArgs { direction: Direction::RUP, length: 1 })
            .upgrade(NewUpgradeArgs { reference: reference!(upgrades.gen_2), direction: Direction::RUP, description: |_| format!("x2 things") })
            .segment(NewSegmentArgs { direction: Direction::UP, length: 1 })
            .upgrade(NewUpgradeArgs { reference: reference!(upgrades.click_2), direction: Direction::LUP, description: |s| format!("+{} things per click\nbased on things per second", s.upgrades.click_2_boost) })
            .segment(NewSegmentArgs { direction: Direction::RUP, length: 1 })
            .segment(NewSegmentArgs { direction: Direction::UP, length: 1 })
        )
        .branch(|s| s
            .segment(NewSegmentArgs { direction: Direction::RUP, length: 1 })
            .upgrade(NewUpgradeArgs { reference: reference!(upgrades.rate_1), direction: Direction::RUP, description: |_| format!("+2 things per second") })
            .segment(NewSegmentArgs { direction: Direction::UP, length: 1 })
            .upgrade(NewUpgradeArgs { reference: reference!(upgrades.gen_1), direction: Direction::LUP, description: |_| format!("x2 things") })
            .segment(NewSegmentArgs { direction: Direction::UP, length: 1 })
            .segment(NewSegmentArgs { direction: Direction::RUP, length: 1 })
            .upgrade(NewUpgradeArgs { reference: reference!(upgrades.rate_2), direction: Direction::RUP, description: |_| format!("x2 things per second") })
        )
    .build(state, renderer);

    ResourceBuilder::new(NewResourceArgs {
        reference: reference!(boosters),
        name: "boosters",
        start_button: NewButtonArgs {
            position: GridPoint::new(0, -9),
            text: |s| {
                if s.boosters > 0 { format!("Boosters: {}\nCost: {} things\nEffect: x{} things", s.boosters, s.boosters_cost, s.boosters_effect) }
                else { format!("???\nCost: {} things", s.boosters_cost) }
            },
            hover: |s| Box::new([
                crate::hover::resource_hover(s, reference!(boosters), "boosters"),
                HoverInfo::Body(format!("cost: {} things\neffect: x{} things", s.boosters_cost, s.boosters_effect)),
            ]),
            enabled: |s| s.things >= s.boosters_cost,
            pressed: |s| {
                s.things -= s.boosters_cost;
                s.boosters += Number::ONE;
            }
        },
        start_direction: Direction::UP,
        growth: Number::from(2).powf(0.5),
        fill_color: parse_color!("#3333ff"),
        back_color: parse_color!("#111266"),
        border_color: parse_color!("#858cad") }
    )
        .segment(NewSegmentArgs { direction: Direction::UP, length: 2 })
        .upgrade(NewUpgradeArgs { reference: reference!(upgrades.pow_1), direction: Direction::RUP, description: |_| format!("^2 things per second") })
    .build(state, renderer);
}

