use crate::game::{Reference, Upgrade as GameUpgrade};
use crate::grid::GridPoint;
use crate::input::{Inputable, upgrade::{Upgrade as InputUpgrade, NewUpgradeArgs as NewInputUpgradeArgs}};
use crate::render::{Renderable, Renderer, upgrade::{Upgrade as RenderUpgrade, NewUpgradeArgs as NewRenderUpgradeArgs}};
use crate::num::{Cost, Quantity};
use crate::state::State;

pub struct NewUpgradeArgs {
    pub position: GridPoint,
    pub description: String,
    pub cost: Cost,
    pub reference: Reference<GameUpgrade>,
    pub resource_reference: Reference<Quantity>,
}

impl State {
    pub fn new_upgrade(&mut self, args:NewUpgradeArgs, renderer:&mut Renderer) {
        let NewUpgradeArgs { position, description, cost, reference, resource_reference } = args;
        let count = &mut self.entities.upgrades;
        self.render_state.upgrades.push(RenderUpgrade::new(*count, NewRenderUpgradeArgs { cost, description, position, reference }, renderer, &self.game_state));
        self.input_state.upgrades.push(InputUpgrade::new(*count, NewInputUpgradeArgs { cost, position, reference, resource_reference }));
        *count += 1;
    }
}
