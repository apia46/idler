use macros::parse_color;
use vello::kurbo::Vec2;
use vello::peniko::Color;
use winit::event::{ElementState, MouseButton};

use crate::game::{GameState, Reference};
use crate::grid::GridPoint;
use crate::num::Quantity;
use crate::render::draw::BuildTextArgs;
use crate::render::{Renderer, Renderable};
use crate::render::hover::{HoverTile, NewHoverTile};
use crate::state::State;

use crate::state::keys::*;

#[derive(Default)]
pub struct HoverState {
    // sticky; doesnt clear if you stop hovering
    pub hover_list: Vec<Hover>,
    pub hover_infos: Vec<HoverInfo>,
}

pub enum Hover {
    ResourceTree(ResourceKey, usize),
    Upgrade(UpgradeKey),
    Button(ButtonKey),
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
pub enum HoverInfo {
    Heading(String),
    Body(String),
}

pub fn update_hover(state:&mut State) {
    let new_hovers = state.input_state.get_hovers();
    if !new_hovers.is_empty() {
        state.hover_state.hover_list = new_hovers;
    }
    let hover_list =  &mut state.hover_state.hover_infos;
    hover_list.clear();
    for hover in &state.hover_state.hover_list {
        match hover {
            Hover::ResourceTree(key, index) => hover_list.extend(state.input_state.resource_trees[key].hover_info(*index, &state.game_state)),
            Hover::Upgrade(key) => hover_list.extend(state.render_state.upgrades[key].hover_info(&state.game_state)),
            Hover::Button(key) => hover_list.extend(state.render_state.buttons[key].hover_info(&state.game_state)),
        }
    }
}

fn text_line(string:&str, font_size:f32, color:Color, renderer:&mut Renderer, pos:&mut Vec2) {
    let text = renderer.build_text(BuildTextArgs { text: &string, font_size, color, center: false });
    renderer.draw_text(*pos, &text, None);
    pos.y += text.height() as f64;
}

impl HoverState {
    pub fn draw(&self, renderer:&mut Renderer) {
        let mut pos:Vec2 = Vec2::new(20., 20.,);
        for hover in &self.hover_infos {
            match hover {
                HoverInfo::Heading(string) => {
                    text_line(string, 18., parse_color!("#ffffff"), renderer, &mut pos);
                },
                HoverInfo::Body(string) => {
                    text_line(string, 12., parse_color!("#dbe3f0"), renderer, &mut pos);
                    pos.y += 6.;
                }
            }
        }
    }
}

pub fn resource_hover(game_state:&GameState, reference:Reference<Quantity>, resource_name:&'static str) -> HoverInfo {
    HoverInfo::Heading(format!("{} {}", game_state.fetch(reference), resource_name))
}
