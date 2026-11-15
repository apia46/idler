use vello::peniko::Color;

use crate::game::{GameState, Reference, Upgrade as GameUpgrade};
use crate::grid::{Direction, GridPoint};
use crate::input::Inputable;
use crate::num::{Cost, Number, Quantity};
use crate::render::Renderable;
use crate::render::{Renderer, resource::{ResourceTree as RenderResourceTree, NewResourceTreeArgs as NewRenderResourceTreeArgs, NewSegmentArgs as NewRenderSegmentArgs}};
use crate::input::resource::{ResourceTree as InputResourceTree, NewResourceTreeArgs as NewInputResourceTreeArgs, NewCellArgs as NewInputCellArgs};
use crate::state::upgrade::NewUpgradeArgs as NewInternalUpgradeArgs;
use crate::state::{State, button::NewButtonArgs};

super::new_key!(ResourceKey);

pub struct NewResourceArgs {
    pub reference: Reference<Quantity>,
    pub name: &'static str,
    pub start_button: NewButtonArgs,
    pub start_direction: Direction,
    pub growth: Number,
    pub fill_color: Color,
    pub back_color: Color,
    pub border_color: Color,
}

pub struct NewSegmentArgs {
    pub direction: Direction,
    pub length: u32,
}

pub struct NewUpgradeArgs {
    pub reference: Reference<GameUpgrade>,
    pub direction: Direction,
    pub description: fn(&GameState) -> String,
}

pub struct ResourceBuilder {
    reference: Reference<Quantity>,
    name: &'static str,
    start_button: NewButtonArgs,
    fill_color: Color,
    back_color: Color,
    border_color: Color,
    segments: Vec<SegmentBuilder>,
    upgrades: Vec<UpgradeBuilder>,
    state: ResourceBuilderState,
}

#[derive(Clone, Copy, Debug)]
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
    growth: Number,
}

#[derive(Clone, Copy)]
struct UpgradeBuilder {
    position: GridPoint,
    description: fn(&GameState) -> String,
    cost: Cost,
    reference: Reference<GameUpgrade>,
    growth: Number,
}

impl ResourceBuilder {
    #[must_use]
    pub fn new(args:NewResourceArgs) -> Self {
        let NewResourceArgs {
            reference, name, start_button, start_direction,
            fill_color, back_color, border_color, growth,
        } = args;
        ResourceBuilder {
            reference, name, start_button, fill_color, back_color, border_color,
            segments: vec![SegmentBuilder {
                start: start_button.position, direction: start_direction, length: 1,
                base: Quantity::from(growth.powf(-0.5)), growth,
            }],
            upgrades: vec![],
            state: ResourceBuilderState { position: start_button.position + start_direction * 1, value: Quantity::ONE, growth }
        }
    }

    #[must_use]
    pub fn segment(mut self, args:NewSegmentArgs) -> Self {
        let NewSegmentArgs { direction, length } = args;
        let half_growth = self.state.growth.powf(0.5);
        let total_growth = self.state.growth.powf(length as f64);
        self.segments.push(SegmentBuilder {
            start: self.state.position, direction, length, base: (self.state.value.to_number() * half_growth).into(), growth: total_growth
        });
        self.state.position += direction * length;
        *self.state.value.as_number() *= total_growth;
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
        let description = description.into();
        self = self.branch(|mut branch| {
            branch = branch.segment(NewSegmentArgs { direction, length: 1 });
            branch.upgrades.push(UpgradeBuilder {
                position: branch.state.position, description, cost: Cost::from(branch.state.value.to_number()),
                reference, growth: branch.state.growth,
            });
            branch
        });
        self
    }

    pub fn build(self, state:&mut State, renderer:&mut Renderer) {
        state.new_button(self.start_button, renderer);

        state.entities.resources.add(|key| {
            let cells = self.segments.iter().flat_map(|s| s.build_input_cell())
                .chain(self.upgrades.iter().map(|u| u.build_input_cell())).collect();

            state.input_state.resource_trees.insert(key, InputResourceTree::new(NewInputResourceTreeArgs {
                resource_name: self.name,
                reference: self.reference,
                cells,
            }));

            state.render_state.resource_trees.insert(key, RenderResourceTree::new(NewRenderResourceTreeArgs {
                key,
                reference: self.reference,
                segments: self.segments.iter().map(|s| s.build_render()).collect(),
                fill_color: self.fill_color,
                back_color: self.back_color,
                border_color: self.border_color,
            }));
        });

        for upgrade_args in &self.upgrades {
            state.new_upgrade(upgrade_args.build_upgrade(self.name, self.reference), renderer);
        }
    }
}

impl SegmentBuilder {
    #[must_use]
    fn build_render(self) -> NewRenderSegmentArgs {
        let SegmentBuilder { start, direction, length, base, growth } = self;
        NewRenderSegmentArgs { start, direction, length, base, growth }
    }

    #[must_use]
    fn build_input_cell(self) -> impl Iterator<Item=NewInputCellArgs> {
        let SegmentBuilder { start, direction, length, base, growth } = self;
            println!("{self:?}");
        (0..length).map(move |i| {
            let base = (base.to_number() * growth.powf((i as f64 - 0.5) / length as f64)).into();
            println!("{base}");
            NewInputCellArgs { pos: start + direction * i, base, growth }
        })
    }
}

impl UpgradeBuilder {
    #[must_use]
    fn build_upgrade(self, resource_name:&'static str, resource_reference:Reference<Quantity>) -> NewInternalUpgradeArgs {
        let UpgradeBuilder { position, description, cost, reference, .. } = self;
        NewInternalUpgradeArgs { position, description, cost, reference, resource_name, resource_reference }
    }

    #[must_use]
    fn build_input_cell(self) -> NewInputCellArgs {
        let UpgradeBuilder { position, cost, growth, .. } = self;
        NewInputCellArgs { pos: position, base: cost.to_number().into(), growth }
    }
}

