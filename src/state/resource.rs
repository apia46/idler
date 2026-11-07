use vello::peniko::Color;

use crate::game::{Reference, Upgrade as GameUpgrade};
use crate::grid::{Direction, GridPoint};
use crate::num::{Cost, Number, Quantity};
use crate::render::Renderable;
use crate::render::{Renderer, resource::{ResourceTree as RenderResourceTree, NewResourceTreeArgs as NewRenderResourceTreeArgs, NewSegmentArgs as NewRenderSegmentArgs}};
use crate::state::upgrade::NewUpgradeArgs as NewInternalUpgradeArgs;
use crate::state::{State, button::NewButtonArgs};

pub struct NewResourceArgs {
    pub reference: Reference<Quantity>,
    pub start_button: NewButtonArgs,
    pub start_direction: Direction,
    pub fill_color: Color,
    pub back_color: Color,
    pub border_color: Color,
}

pub struct NewSegmentArgs {
    pub direction: Direction,
    pub length: u32,
    pub growth: Number,
}

pub struct NewUpgradeArgs {
    pub reference: Reference<GameUpgrade>,
    pub direction: Direction,
    pub description: String,
}

pub struct ResourceBuilder {
    reference: Reference<Quantity>,
    start_button: NewButtonArgs,
    fill_color: Color,
    back_color: Color,
    border_color: Color,
    segments: Vec<SegmentBuilder>,
    upgrades: Vec<NewInternalUpgradeArgs>,
    state: ResourceBuilderState,
}

#[derive(Clone, Copy)]
struct SegmentBuilder {
    start: GridPoint,
    direction: Direction,
    length: u32,
    base: Quantity,
    growth: Number,
}

#[derive(Clone, Copy)]
struct ResourceBuilderState {
    position: GridPoint,
    value: Quantity,
}

impl ResourceBuilder {
    #[must_use]
    pub fn new(args:NewResourceArgs) -> Self {
        let NewResourceArgs {
            reference, start_button, start_direction,
            fill_color, back_color, border_color
        } = args;
        ResourceBuilder {
            reference, start_button, fill_color, back_color, border_color,
            segments: vec![SegmentBuilder {
                start: start_button.position, direction: start_direction, length: 1,
                base: Quantity::new(0.5), growth: Number::new(2.),
            }],
            upgrades: vec![],
            state: ResourceBuilderState { position: start_button.position + start_direction * 1, value: Quantity::ONE }
        }
    }

    #[must_use]
    pub fn segment(mut self, args:NewSegmentArgs) -> Self {
        let NewSegmentArgs { direction, length, growth } = args;
        self.segments.push(SegmentBuilder { start: self.state.position, direction, length, base: self.state.value, growth });
        self.state.position += direction * length;
        *self.state.value.as_number() *= growth;
        self
    }

    #[must_use]
    pub fn branch(mut self, branch:impl FnOnce(Self)->Self) -> Self {
        let old_state = self.state;
        self = branch(self);
        self.state = old_state;
        self
    }

    #[must_use]
    pub fn upgrade(mut self, args:NewUpgradeArgs) -> Self {
        let NewUpgradeArgs { reference, direction, description } = args;
        self = self.branch(|mut branch| {
            branch = branch.segment(NewSegmentArgs { direction, length: 1, growth: 1.into() });
            branch.upgrades.push(NewInternalUpgradeArgs {
                position: branch.state.position, description, cost: Cost::from(branch.state.value.to_number()),
                reference, resource_reference: branch.reference
            });
            branch
        });
        self
    }

    pub fn build(self, state:&mut State, renderer:&mut Renderer) {
        state.new_button(self.start_button, renderer);

        let count = &mut state.entities.resources;

        state.render_state.resource_trees.push(RenderResourceTree::new(*count, NewRenderResourceTreeArgs {
            reference: self.reference,
            segments: self.segments.iter().map(|s| s.build_render()).collect(),
            fill_color: self.fill_color,
            back_color: self.back_color,
            border_color: self.border_color,
        }, renderer, &state.game_state));
        *count += 1;

        for upgrade_args in self.upgrades {
            state.new_upgrade(upgrade_args, renderer);
        }
    }
}

impl SegmentBuilder {
    #[must_use]
    fn build_render(self) -> NewRenderSegmentArgs {
        let SegmentBuilder { start, direction, length, base, growth } = self;
        NewRenderSegmentArgs { start, direction, length, base, growth }
    }
}

