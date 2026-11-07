use winit::event::{ElementState, MouseButton};

use crate::game::{GameState, Reference, Upgrade as GameUpgrade};
use crate::grid::GridPoint;
use crate::input::button::InteractState;
use crate::input::{Inputable, MouseInputResponse, UserState};
use crate::num::{Cost, Quantity};
use crate::render::camera::Camera;

pub struct Upgrade {
    #[expect(unused)]
    id: usize,
    position: GridPoint,
    state: InteractState,
    reference: Reference<GameUpgrade>,
    resource_reference: Reference<Quantity>,
    cost: Cost,
}

pub struct NewUpgradeArgs {
    pub position: GridPoint,
    pub reference: Reference<GameUpgrade>,
    pub resource_reference: Reference<Quantity>,
    pub cost: Cost,
}

impl Upgrade {
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
    type NewArgs = NewUpgradeArgs;
    fn new(id: usize, args: NewUpgradeArgs) -> Self {
        let NewUpgradeArgs { position, reference, resource_reference, cost } = args;
        Upgrade { id, position, state: InteractState::Disabled, reference, resource_reference, cost }
    }

    fn update(&mut self, user_state:&UserState, game_state:&GameState, camera:&Camera) {
        self.state = if self.disable(game_state) {
            InteractState::Disabled
        } else {
            match (self.state, GridPoint::nearest(camera.to_world(user_state.mouse_position)) == self.position) {
                (InteractState::None | InteractState::Disabled, true) => InteractState::Hovered,
                (_, false) => InteractState::None,
                (state, _) => state,
            }
        }
    }

    fn mouse_input(
        &mut self, button_state:ElementState, mouse_button:MouseButton,
        user_state:&UserState, camera:&Camera, game_state:&mut GameState
    ) -> MouseInputResponse {
        let hovered = matches!(self.state, InteractState::Hovered | InteractState::Pressed);
        if !matches!(mouse_button, MouseButton::Left) { return MouseInputResponse { hovered, consumed: false }; };
        let consumed = match (self.state, button_state) {
            (InteractState::Hovered, ElementState::Pressed) => {
                self.state = InteractState::Pressed; true
            }
            (InteractState::Pressed, ElementState::Released) => {
                self.effect(game_state);
                self.state = InteractState::None;
                self.update(user_state, game_state, camera);
                true
            },
            _ => false,
        };
        MouseInputResponse { hovered, consumed }
    }
}

