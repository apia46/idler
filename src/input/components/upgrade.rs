use winit::event::{ElementState, MouseButton};

use crate::game::{GameState, Reference, Upgrade as GameUpgrade};
use crate::grid::GridPoint;
use crate::hover::Hover;
use crate::input::button::{self, InteractState};
use crate::input::{Inputable, MouseInputContext, MouseInputResponse, UpdateContext};
use crate::num::{Cost, Quantity};
use crate::state::upgrade::UpgradeKey;

pub struct Upgrade {
    key: UpgradeKey,
    position: GridPoint,
    state: InteractState,
    reference: Reference<GameUpgrade>,
    resource_reference: Reference<Quantity>,
    cost: Cost,
}

pub struct NewUpgradeArgs {
    pub key: UpgradeKey,
    pub position: GridPoint,
    pub reference: Reference<GameUpgrade>,
    pub resource_reference: Reference<Quantity>,
    pub cost: Cost,
}

impl Upgrade {
    pub fn new(args: NewUpgradeArgs) -> Self {
        let NewUpgradeArgs { key, position, reference, resource_reference, cost } = args;
        Upgrade { key, position, state: InteractState::Disabled, reference, resource_reference, cost }
    }

    fn disable(&self, game_state:&GameState) -> bool {
        game_state.fetch(self.reference).purchased || game_state.fetch(self.resource_reference) < self.cost
    }

    fn effect(&self, game_state:&mut GameState) {
        game_state.fetch_mut(self.reference).purchased = true;
        *game_state.fetch_mut(self.resource_reference) -= self.cost;
    }

    pub fn state(&self) -> InteractState {
        self.state
    }
}

impl Inputable for Upgrade {
    fn update(&mut self, ctx:&mut UpdateContext) {
        self.state = button::next_state(self.state, self.position, ctx.user_state, self.disable(ctx.game_state));
    }

    fn hovered(&self) -> Option<Hover> {
        button::hovered(self.state).then(|| Hover::Upgrade(self.key))
    }

    fn mouse_input(&mut self, ctx:&mut MouseInputContext) -> MouseInputResponse {
        let hovered = matches!(self.state, InteractState::Hovered | InteractState::Pressed);
        if !matches!(ctx.mouse_button, MouseButton::Left) { return MouseInputResponse { hovered, consumed: false }; };
        let consumed = match (self.state, ctx.button_state) {
            (InteractState::Hovered, ElementState::Pressed) => {
                self.state = InteractState::Pressed; true
            }
            (InteractState::Pressed, ElementState::Released) => {
                self.effect(ctx.game_state);
                self.state = button::next_state(InteractState::None, self.position, ctx.user_state, self.disable(ctx.game_state));
                true
            },
            _ => false,
        };
        MouseInputResponse { hovered, consumed }
    }
}

