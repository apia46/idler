use macros::parse_color;
use vello::peniko::Color;

use crate::grid::GridPoint;
use crate::render::{Renderable, Renderer, ProcessContext, camera::Camera, draw::{Style, DrawHexagonArgs}};
use crate::util::Remap;

const FADE_TIME:f64 = 0.2;

const COLOR:Color = parse_color!("#9cb2c9");

pub struct HoverTile {
    position: GridPoint,
    state: HoverTileState,
}

enum HoverTileState {
    Hover { age: f64 },
    Fading { age: f64 },
}

pub struct NewHoverTile {
    pub position: GridPoint,
}

impl HoverTile {
    pub fn renew(&mut self) {
        self.state = HoverTileState::Hover { age: 0. };
    }

    pub fn unhover(&mut self) {
        self.state = HoverTileState::Fading { age: 0. };
    }

    pub fn delete(&self) -> bool {
        if let HoverTileState::Fading { age } = self.state {
            age > FADE_TIME
        } else { false }
    }
}

impl<'a> Renderable<'a> for HoverTile {
    type NewArgs = NewHoverTile;
    fn new(args:Self::NewArgs) -> Self {
        let NewHoverTile { position } = args;
        HoverTile { position, state: HoverTileState::Hover { age: 0. } }
    }

    fn process(&mut self, ctx:&mut ProcessContext) {
        match &mut self.state {
            HoverTileState::Hover { age } => { *age += ctx.delta_time },
            HoverTileState::Fading { age } => { *age += ctx.delta_time },
        }
    }

    fn draw(&self, renderer:&mut Renderer, camera:&Camera) {
        let alpha = match self.state {
            HoverTileState::Hover { age } => { age.remap((0., 0.3), (1., 0.5)) }
            HoverTileState::Fading { age } => { age.remap((0., FADE_TIME), (0.5, 0.)) }
        };
        renderer.draw_hexagon(DrawHexagonArgs {
            point: self.position,
            style: Style { stroke: Some((4., COLOR.with_alpha(alpha as f32))), fill: None }
        }, camera);
    }
}

