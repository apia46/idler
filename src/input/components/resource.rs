use crate::game::Reference;
use crate::grid::GridPoint;
use crate::hover::HoverInfo;
use crate::input::{HoverContext, Inputable};
use crate::num::{Number, Quantity};

pub struct ResourceTree {
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
    pub resource_name: &'static str,
    pub reference: Reference<Quantity>,
    pub cells: Vec<NewCellArgs>
}

pub struct NewCellArgs {
    pub pos: GridPoint,
    pub base: Quantity,
    pub growth: Number,
}

impl Inputable for ResourceTree {
    type NewArgs = NewResourceTreeArgs;

    fn new(args:Self::NewArgs) -> Self {
        let NewResourceTreeArgs { resource_name, reference, cells:cell_args } = args;
        let mut cells = Vec::with_capacity(cell_args.len());
        for NewCellArgs { pos, base, growth } in cell_args {
            cells.push(Cell { pos, base, growth });
        }
        Self {
            resource_name, reference, cells
        }
    }

    fn hover(&self, ctx:&mut HoverContext) -> Option<HoverInfo> {
        for cell in &self.cells {
            if ctx.user_state.mouse_grid_point == cell.pos {
                return Some(HoverInfo {
                    title: format!("{} {}", ctx.game_state.fetch(self.reference), self.resource_name),
                    body: format!("value: {}\nx{} per tile", cell.base, cell.growth),
                });
            }
        }
        None
    }
}

