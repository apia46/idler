use macros::parse_color;
use vello::peniko::Color;

use crate::game::{GameState, Reference, Upgrade as GameUpgrade};
use crate::grid::GridPoint;
use crate::input::{InputState, button::InteractState};
use crate::num::Cost;
use crate::render::draw::{DrawHexagonArgs, Style};
use crate::render::{camera::Camera, draw::{BuildTextArgs, TextLayout}, Renderable, Renderer};

const FONT_SIZE:f32 = 12.;
const BORDER_COLOR:Color = parse_color!("#444444");

const TEXT_COLOR:Color = parse_color!("#ffffff");
const BG_COLOR:Color = parse_color!("#666666");

const HOVERED_COLOR:Color = parse_color!("#808080");

const PRESSED_COLOR:Color = parse_color!("#555555");

const DISABLED_COLOR:Color = parse_color!("#333333");
const DISABLED_TEXT_COLOR:Color = parse_color!("#999999");

const PURCHASED_TEXT_COLOR:Color = parse_color!("#5f5529");
const PURCHASED_BG_COLOR:Color = parse_color!("#e5d36f");

pub struct Upgrade {
    id: usize,
    position: GridPoint,
    resource_name: &'static str,
    description: fn(&GameState) -> String,
    cost: Cost,
    reference: Reference<GameUpgrade>,
    layout: TextLayout,
    bg_color: Color,
}

pub struct NewUpgradeArgs {
    pub position: GridPoint,
    pub resource_name: &'static str,
    pub description: fn(&GameState) -> String,
    pub cost: Cost,
    pub reference: Reference<GameUpgrade>,
}

#[must_use]
fn build_text(game_state:&GameState, renderer:&mut Renderer, reference:Reference<GameUpgrade>, resource_name:&'static str, mut description:String, cost:Cost, state:InteractState) -> TextLayout {
    let purchased = game_state.fetch(reference).purchased;
    if !purchased { description += &format!("\nCost: {cost} {resource_name}") }
    renderer.build_text(BuildTextArgs {
        text: &description, font_size: FONT_SIZE,
        color: if let InteractState::Disabled = state { if purchased { PURCHASED_TEXT_COLOR } else { DISABLED_TEXT_COLOR } } else { TEXT_COLOR },
        center: true,
    })
}

impl Renderable for Upgrade {
    type NewArgs = NewUpgradeArgs;
    fn new(id:usize, args:NewUpgradeArgs, renderer:&mut Renderer, state:&GameState) -> Self {
        let NewUpgradeArgs { position, resource_name, description, cost, reference } = args;
        let layout = build_text(state, renderer, reference, resource_name, (description)(state), cost, InteractState::Disabled);

        Upgrade {
            id, position, resource_name, description, cost, reference,
            layout, bg_color: BG_COLOR,
        }
    }

    fn update(&mut self, renderer:&mut Renderer, game_state:&GameState, input_state:&InputState) {
        let interact_state = input_state.upgrades[self.id].state();
        self.bg_color = if game_state.fetch(self.reference).purchased { PURCHASED_BG_COLOR }
        else {
            match interact_state {
                InteractState::None => BG_COLOR,
                InteractState::Hovered => HOVERED_COLOR,
                InteractState::Pressed => PRESSED_COLOR,
                InteractState::Disabled => DISABLED_COLOR,
            }
        };
        self.layout = build_text(game_state, renderer, self.reference, self.resource_name, (self.description)(game_state), self.cost, interact_state);
    }

    fn draw(&self, renderer:&mut Renderer, camera:&Camera) {
        let style = Style {
            stroke: Some((4., BORDER_COLOR)), fill: Some(self.bg_color)
        };
        renderer.draw_hexagon(DrawHexagonArgs { point: self.position, style }, camera);
        renderer.draw_text(self.position.to_point().to_vec2(), &self.layout, camera)
    }
}

