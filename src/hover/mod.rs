use macros::parse_color;
use vello::kurbo::Vec2;
use vello::peniko::Color;
use winit::event::{ElementState, MouseButton};

use crate::grid::GridPoint;
use crate::input::Hover;
use crate::render::draw::BuildTextArgs;
use crate::render::{Renderer, Renderable};
use crate::render::hover::{HoverTile, NewHoverTile};
use crate::state::State;

#[derive(Default)]
pub struct HoverState {
    // sticky; doesnt clear if you stop hovering
    pub hover_list: Vec<Hover>,
    pub hover_infos: Vec<HoverInfo>,
}

pub fn update(state: &mut State) {
    state.render_state.hover_tiles.retain(|_, tile| {
        !tile.delete()
    });
    update_hover(state);
}

pub fn mouse_moved(from:GridPoint, to:GridPoint, state:&mut State) {
    if from != to {
        state.render_state.hover_tiles.entry(from).and_modify(|tile| { tile.unhover(); });
        state.render_state.hover_tiles.insert(to, HoverTile::new(NewHoverTile { position: to }));
    }
    update_hover(state);
}

pub fn mouse_input(button_state:ElementState, mouse_button:MouseButton, state:&mut State) {
    if let (ElementState::Pressed, MouseButton::Left) = (button_state, mouse_button) {
        state.render_state.hover_tiles.entry(state.input_state.user_state.mouse_grid_point).and_modify(|tile| { tile.renew(); });
    }
}

#[derive(Debug)]
pub struct HoverInfo {
    pub title: String,
    pub body: String,
}

pub fn update_hover(state: &mut State) {
    let new_hovers = state.input_state.get_hovers();
    if !new_hovers.is_empty() {
        state.hover_state.hover_list = new_hovers;
    }
    state.hover_state.hover_infos = state.hover_state.hover_list.iter().map(|hover| {
        match hover {
            Hover::ResourceTree(key, index) => state.input_state.resource_trees[key].hover_info(*index, &state.game_state)
        }
    }).collect();
}

impl HoverState {
    pub fn draw(&self, renderer: &mut Renderer) {
        let mut pos:Vec2 = Vec2::new(20., 20.,);
        for hover in &self.hover_infos {
            let title = renderer.build_text(BuildTextArgs { text: &hover.title, font_size: 18., color: Color::WHITE, center: false });
            renderer.draw_text(pos, &title, None);
            pos.y += title.height() as f64;
            let body = renderer.build_text(BuildTextArgs { text: &hover.body, font_size: 12., color: parse_color!("#dbe3f0"), center: false });
            renderer.draw_text(pos, &body, None);
            pos.y += body.height() as f64;
        }
    }
}

