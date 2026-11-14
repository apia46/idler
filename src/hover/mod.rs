use winit::event::{ElementState, MouseButton};

use crate::grid::GridPoint;
use crate::render::Renderable;
use crate::render::hover::{HoverTile, NewHoverTile};
use crate::state::State;

pub fn update(state: &mut State) {
    state.render_state.hover_tiles.retain(|_, tile| {
        !tile.delete()
    });
}

pub fn mouse_moved(from:GridPoint, to:GridPoint, state:&mut State) {
    if from != to {
        state.render_state.hover_tiles.entry(from).and_modify(|tile| { tile.unhover(); });
        state.render_state.hover_tiles.insert(to, HoverTile::new(NewHoverTile { position: to }));
    }
}

pub fn mouse_input(button_state:ElementState, mouse_button:MouseButton, state:&mut State) {
    if let (ElementState::Pressed, MouseButton::Left) = (button_state, mouse_button) {
        state.render_state.hover_tiles.entry(state.input_state.user_state.mouse_grid_point).and_modify(|tile| { tile.renew(); });
    }
}

