use crate::game::{GameState, Reference};
use crate::grid::GridPoint;
use crate::hover::{Hover, HoverInfo};
use crate::input::Inputable;
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
    hovered: bool,
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
            cells.push(Cell { pos, base, growth, hovered: false });
        }
        Self {
            key, resource_name, reference, cells
        }
    }

    pub fn hover_info(&self, index:usize, game_state:&GameState) -> [HoverInfo; 2] {
        let cell = &self.cells[index];
        [
            crate::hover::resource_hover(game_state, self.reference, self.resource_name),
            HoverInfo::Body(format!("value at {}: {}\nx{} per tile", cell.pos, cell.base, cell.growth)),
        ]
    }
}

impl Inputable for ResourceTree {
    fn update(&mut self, ctx:&mut crate::input::UpdateContext) {
        for cell in &mut self.cells {
             cell.hovered = ctx.user_state.mouse_grid_point == cell.pos;
        }
    }
    fn hovered(&self) -> Option<Hover> {
        for (i, cell) in self.cells.iter().enumerate() {
            if cell.hovered {
                return Some(Hover::ResourceTree(self.key, i));
            }
        }
        None
    }
}

