use crate::game::{GameState, Reference};
use crate::grid::GridPoint;
use crate::hover::HoverInfo;
use crate::input::{Hover, HoverContext, HoverInfoContext, Inputable};
use crate::num::{Number, Quantity};
use crate::state::keys::ResourceKey;

pub struct ResourceTree {
    key: ResourceKey,
    resource_name: &'static str,
    reference: Reference<Quantity>,
    cells: Vec<Cell>
}

struct Cell {
    pos: GridPoint,
    base: Quantity,
    growth: Number,
}

pub struct NewResourceTreeArgs {
    pub key: ResourceKey,
    pub resource_name: &'static str,
    pub reference: Reference<Quantity>,
    pub cells: Vec<NewCellArgs>
}

pub struct NewCellArgs {
    pub pos: GridPoint,
    pub base: Quantity,
    pub growth: Number,
}

impl ResourceTree {
    pub fn new(args:NewResourceTreeArgs) -> Self {
        let NewResourceTreeArgs { key, resource_name, reference, cells:cell_args } = args;
        let mut cells = Vec::with_capacity(cell_args.len());
        for NewCellArgs { pos, base, growth } in cell_args {
            cells.push(Cell { pos, base, growth });
        }
        Self {
            key, resource_name, reference, cells
        }
    }

    pub fn hover_info(&self, index:usize, game_state:&GameState) -> HoverInfo {
        let cell = &self.cells[index];
        HoverInfo {
            title: format!("{} {}", game_state.fetch(self.reference), self.resource_name),
            body: format!("value: {}\nx{} per tile", cell.base, cell.growth),
        }
    }
}

impl Inputable for ResourceTree {
    fn hovered(&self, ctx:&mut HoverContext) -> Option<Hover> {
        for (i, cell) in self.cells.iter().enumerate() {
            if ctx.user_state.mouse_grid_point == cell.pos {
                return Some(Hover::ResourceTree(self.key, i));
            }
        }
        None
    }
}

